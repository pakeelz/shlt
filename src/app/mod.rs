mod events;
mod render;

use std::time::Duration;

use crossterm::{
    event::{Event, EventStream, KeyCode},
    terminal,
};
use futures_util::StreamExt;
use ratatui::{DefaultTerminal, widgets::ListState};

use crate::{
    api::{self, Schedule, get_jadwal_sholat, get_possible_kabkota},
    error::AppError,
    storage,
};

pub struct App {
    state: AppState,
    previous_state: Option<AppState>,
    data_shalat: Option<Schedule>,
    mode: Mode,
    exit: bool,
    tick: usize,
    province_items: Vec<String>,
    province_items_filter: Option<Vec<String>>,
    filter: Option<String>,
    province_selected: Option<String>,
    kabkota_items: Option<Vec<String>>,
    kabkota_items_filter: Option<Vec<String>>,
    kabkota_selected: Option<String>,
    province_state: ListState,
    message: Option<String>,
}

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum AppState {
    Response(Schedule),
    WaitingResponse,
    ResponseKabKota,
    ResponseKabKotaFilter,
    WaitingKabkota,
    ShowProvince,
    ProvinceFilter,
    UpdatingConfig,
    Error(AppError),
}

#[derive(PartialEq)]
pub enum Mode {
    Display,
    Result,
    Filter,
}

impl App {
    pub fn new() -> Self {
        let data_province = api::PROVINSI
            .to_vec()
            .iter()
            .map(|&f| f.to_string())
            .collect::<Vec<String>>();

        Self {
            state: AppState::ShowProvince,
            previous_state: None,
            data_shalat: None,
            mode: Mode::Display,
            exit: false,
            tick: 0,
            province_items: data_province, // Data Province
            province_items_filter: None,
            filter: None,            // Popup filter items
            province_selected: None, // Selected Province
            kabkota_items: None,
            kabkota_items_filter: None,
            kabkota_selected: None,
            province_state: ListState::default().with_selected(Some(0)),
            message: None,
        }
    }

    pub fn response_state(&mut self, shalat: Schedule) {
        self.state = AppState::Response(shalat);
        self.mode = Mode::Result;
    }

    pub fn set_updating_config_state(&mut self) {
        self.state = AppState::UpdatingConfig;
        self.tick = 2;
    }

    fn reset(&mut self) {
        self.state = AppState::ShowProvince;
        self.data_shalat = None;
        self.mode = Mode::Display;
        self.tick = 0;
        self.province_items_filter = None;
        self.filter = None;
        self.province_selected = None;
        self.kabkota_selected = None;
        self.kabkota_items = None;
        self.kabkota_items_filter = None;
        self.province_state = ListState::default().with_selected(Some(0));
    }

    pub fn set_province_items_filter(&mut self, data: Vec<String>) {
        self.province_items_filter = Some(data);
    }

    fn set_kabkota_items_filter(&mut self, data: Vec<String>) {
        self.kabkota_items_filter = Some(data);
    }

    fn set_data_shalat(&mut self, shalat: Schedule) {
        self.data_shalat = Some(shalat);
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        let mut reader = EventStream::new();
        let mut interval = tokio::time::interval(Duration::from_millis(500));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        while !self.exit {
            self.handle_terminal_size()?;
            match &self.state {
                AppState::Response(shalat) => {
                    self.set_data_shalat(shalat.clone());
                    self.render_response(terminal)?
                }
                AppState::ShowProvince | AppState::ProvinceFilter => {
                    self.render_all_province(terminal)?
                }
                AppState::WaitingResponse | AppState::WaitingKabkota => {
                    self.render_waiting_response(terminal)?;
                }
                AppState::ResponseKabKota | AppState::ResponseKabKotaFilter => {
                    self.render_all_kabkota(terminal)?;
                }
                AppState::UpdatingConfig => {
                    self.render_updating_config(terminal)?;
                }
                AppState::Error(e) => {
                    self.render_error(terminal, e)?;
                }
            }

            let province = &self
                .province_selected
                .clone()
                .unwrap_or_else(|| "Dki Jakarta".to_string());

            let kabkota = &self
                .kabkota_selected
                .clone()
                .unwrap_or_else(|| "Kota Jakarta".to_string());

            tokio::select! {
                _ = interval.tick(), if self.state == AppState::WaitingResponse
                || self.state == AppState::WaitingKabkota
                || self.state == AppState::UpdatingConfig => {
                    self.tick += 1;
                }
                _ = tokio::time::sleep(Duration::from_millis(1500)), if self.message.is_some() => {
                    self.message = None;
                }
                result_shalat = get_jadwal_sholat(province, kabkota), if self.state == AppState::WaitingResponse
                || self.state == AppState::UpdatingConfig => {
                    match result_shalat {
                        Ok(shalat) => {
                            if self.state == AppState::UpdatingConfig {
                                storage::write_config(&shalat)?;
                            }

                            self.state = AppState::Response(shalat);
                            self.mode = Mode::Result;
                            self.tick = 0;
                        },
                        Err(e) => {
                            self.state = AppState::Error(e);
                        }
                    }
                }
                result = get_possible_kabkota(province), if self.state == AppState::WaitingKabkota => {
                    match result {
                        Ok(kabkota) => {
                            self.kabkota_items = Some(kabkota.data);
                            self.state = AppState::ResponseKabKota;
                            self.tick = 0;
                        },
                        Err(e) => {
                            self.state = AppState::Error(e);
                        }
                    }
                }
                Some(Ok(event)) = reader.next() => {
                    if let Event::Key(key) = event {
                        match key.code {
                            KeyCode::Char('q') if self.mode != Mode::Filter => {
                                self.exit = true;
                            }
                            KeyCode::Char('n') if self.mode == Mode::Result => {
                                self.reset();
                            }
                            KeyCode::Char('d') if self.mode == Mode::Result => {
                                if let Some(shalat) = &self.data_shalat {
                                    if let Err(e) = storage::write_config(shalat) {
                                        self.state = AppState::Error(e);
                                    } else {
                                        self.message = Some(String::from("successfully created default"));
                                    }
                                }
                            }
                            KeyCode::Left
                                if self.state == AppState::ResponseKabKota
                                || self.state == AppState::ResponseKabKotaFilter => {
                                    self.reset();
                            },
                            KeyCode::Up
                                if self.state == AppState::ShowProvince
                                || self.state == AppState::ProvinceFilter
                                || self.state == AppState::ResponseKabKota
                                || self.state == AppState::ResponseKabKotaFilter => {
                                    self.prev();
                            },
                            KeyCode::Down
                                if self.state == AppState::ShowProvince
                                || self.state == AppState::ProvinceFilter
                                || self.state == AppState::ResponseKabKota
                                || self.state == AppState::ResponseKabKotaFilter => {
                                    self.next();
                            },
                            KeyCode::Enter
                                if self.state == AppState::ShowProvince
                                || self.state == AppState::ProvinceFilter
                                || self.state == AppState::ResponseKabKota
                                || self.state == AppState::ResponseKabKotaFilter => {
                                    self.enter();
                                    if self.state == AppState::ShowProvince || self.state == AppState::ProvinceFilter {
                                        self.state = AppState::WaitingKabkota;
                                        self.province_items_filter = None;
                                    } else {
                                        self.kabkota_items_filter = None;
                                        self.state = AppState::WaitingResponse;
                                    }
                                    self.filter = None;
                                    self.mode = Mode::Display;
                                    self.province_state = ListState::default().with_selected(Some(0));
                            }
                            KeyCode::Char('f') if self.state == AppState::ShowProvince
                                || self.state == AppState::ResponseKabKota => {
                                    self.mode = Mode::Filter;
                                    if self.state == AppState::ShowProvince {
                                        self.state = AppState::ProvinceFilter;
                                    } else if self.state == AppState::ResponseKabKota {
                                        self.state = AppState::ResponseKabKotaFilter
                                    }
                                    self.filter = Some(String::new());
                                    self.province_state = ListState::default().with_selected(Some(0));
                            },
                            KeyCode::Esc if self.mode == Mode::Filter => {
                                self.mode = Mode::Display;
                                if self.state == AppState::ProvinceFilter {
                                    self.state = AppState::ShowProvince;
                                    self.province_items_filter = None;
                                } else if self.state == AppState::ResponseKabKotaFilter {
                                    self.state = AppState::ResponseKabKota;
                                    self.kabkota_items_filter = None;
                                }
                                self.filter = None;
                            },
                            KeyCode::Char(c) if self.state == AppState::ProvinceFilter
                                || self.state == AppState::ResponseKabKotaFilter => {
                                    if let Some(filter) = &self.filter {
                                        let mut buffer = filter.clone();
                                        buffer.push(c);

                                        let filtered_data = if self.state == AppState::ProvinceFilter {
                                            &self.province_items
                                            .iter()
                                            .filter(|&f| f.to_lowercase().contains(&buffer.to_lowercase()))
                                            .map(|f| f.clone())
                                            .collect::<Vec<String>>()
                                        } else {
                                            &self.kabkota_items.clone().unwrap_or_else(|| Vec::new())
                                            .iter()
                                            .filter(|&f| f.to_lowercase().contains(&buffer.to_lowercase()))
                                            .map(|f| f.clone())
                                            .collect::<Vec<String>>()
                                        };

                                        if self.state == AppState::ProvinceFilter {
                                            self.set_province_items_filter(filtered_data.clone());
                                        } else if self.state == AppState::ResponseKabKotaFilter {
                                            self.set_kabkota_items_filter(filtered_data.clone());
                                        }
                                        self.filter = Some(buffer);
                                    }
                            },
                            KeyCode::Backspace if self.state == AppState::ProvinceFilter
                                || self.state == AppState::ResponseKabKotaFilter => {
                                    if let Some(filter) = &self.filter {
                                        let mut buffer = filter.clone();
                                        buffer.pop();

                                        let filtered_data = if self.state == AppState::ProvinceFilter {
                                            &self.province_items
                                            .iter()
                                            .filter(|&f| f.to_lowercase().contains(&buffer.to_lowercase()))
                                            .map(|f| f.clone())
                                            .collect::<Vec<String>>()
                                        } else {
                                            &self.kabkota_items.clone().unwrap_or_else(|| Vec::new())
                                            .iter()
                                            .filter(|&f| f.to_lowercase().contains(&buffer.to_lowercase()))
                                            .map(|f| f.clone())
                                            .collect::<Vec<String>>()
                                        };

                                        if self.state == AppState::ProvinceFilter {
                                            self.set_province_items_filter(filtered_data.clone());
                                        } else if self.state == AppState::ResponseKabKotaFilter {
                                            self.set_kabkota_items_filter(filtered_data.clone());
                                        }
                                        self.filter = Some(buffer);
                                    }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_terminal_size(&mut self) -> Result<(), AppError> {
        let (cols, rows) = terminal::size()?;
        if rows < 14 || cols < 28 {
            self.previous_state = Some(self.state.clone());
            self.state = AppState::Error(AppError::TerminalToSmall)
        } else {
            if let Some(state) = &self.previous_state {
                self.state = state.clone();
                self.previous_state = None;
            }
        }
        Ok(())
    }
}

// IU
fn time_now() -> usize {
    let local = chrono::Local::now().format("%d").to_string();
    local.parse().unwrap()
}
