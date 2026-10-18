use crate::{
    app::{App, time_now},
    error::AppError,
};

use crossterm::terminal;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{
        Color::{self},
        Style, Stylize,
    },
    widgets::{Block, List, ListItem, Padding, Paragraph, Scrollbar, ScrollbarState, Wrap},
};

impl App {
    pub fn render_response(&self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        let (cols, rows) = terminal::size()?;
        if let Some(shalat) = &self.data_shalat {
            terminal.draw(|frame| {
                let header = &shalat.kabkota;
                let jadwal = &shalat.data[time_now()];

                let (x, y, width, height) = (
                    (cols.saturating_sub(36)) / 2,
                    (rows.saturating_sub(14)) / 2,
                    36u16,
                    14u16,
                );

                let rect = Rect::new(x, y, width, height);
                let [top, bottom] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Fill(1)])
                    .areas(rect);

                let [main, fotter] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Fill(1), Constraint::Length(4)])
                    .areas(bottom);

                let header = Paragraph::new(header.to_string())
                    .block(Block::bordered())
                    .centered();

                let content = Paragraph::new(vec![
                    format!("Subuh: {}", jadwal.subuh).into(),
                    format!("Dzuhur: {}", jadwal.dzuhur).into(),
                    format!("Ashar: {}", jadwal.ashar).into(),
                    format!("Maghrib: {}", jadwal.maghrib).into(),
                    format!("Isya: {}", jadwal.isya).into(),
                ])
                .block(Block::bordered())
                .centered();

                let fot = Paragraph::new(vec![
                    "Press N for new session".into(),
                    "Press D to make it default".into(),
                ])
                .block(Block::bordered().title_bottom(" <q> Exit "))
                .centered();

                frame.render_widget(header, top);
                frame.render_widget(content, main);
                frame.render_widget(fot, fotter);
                if let Ok(_) = self.handle_message(frame) {}
            })?;
        } else {
            return Err(AppError::Other("UnkwonError".to_string()));
        }
        Ok(())
    }

    pub fn render_waiting_response(&self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        let (cols, rows) = terminal::size()?;
        terminal.draw(|frame| {
            let (x, y, width, height) = (
                (cols.saturating_sub(30)) / 2,
                (rows.saturating_sub(3)) / 2,
                30u16,
                3u16,
            );

            let rect = Rect::new(x, y, width, height);

            let message = format!("Waiting response{}", ".".repeat(self.tick % 5));

            let paragraph = Paragraph::new(message)
                .block(Block::bordered())
                .wrap(Wrap { trim: true })
                .centered();

            frame.render_widget(paragraph, rect);
        })?;
        Ok(())
    }

    pub fn render_updating_config(&self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        let (cols, rows) = terminal::size()?;
        terminal.draw(|frame| {
            let message = format!("Updating{}", ".".repeat(self.tick % 5));

            let width = message.len() as u16 + 6;
            let height = 3u16;
            let (x, y) = (
                (cols.saturating_sub(width)) / 2,
                (rows.saturating_sub(height)) / 2,
            );

            let rect = Rect::new(x, y, width, height);

            let paragraph = Paragraph::new(message).block(Block::bordered()).centered();

            frame.render_widget(paragraph, rect);
        })?;

        Ok(())
    }

    pub fn render_all_province(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        let (cols, rows) = terminal::size()?;
        terminal.draw(|frame| {
            let (x, y, width, height) = (
                (cols.saturating_sub(36)) / 2,
                (rows.saturating_sub(13)) / 2,
                36u16,
                13u16,
            );

            let rect = Rect::new(x, y, width, height);

            let [top, bottom] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Fill(1)])
                .areas(rect);

            let items = match &self.province_items_filter {
                Some(filtered_data) => &filtered_data
                    .iter()
                    .map(|f| ListItem::new(f.as_str()))
                    .collect(),
                None => &self
                    .province_items
                    .iter()
                    .map(|f| ListItem::new(f.as_str()))
                    .collect::<Vec<ListItem>>(),
            };

            let list = List::new(items.clone())
                .block(
                    Block::bordered()
                        .title_bottom(" <q> Exit ")
                        .title_bottom("  Up ")
                        .title_bottom("  Down ")
                        .title_top(" <f> filter ")
                        .title_top(" <Enter> Select ")
                        .padding(Padding::new(1, 2, 0, 0)),
                )
                .highlight_symbol(">> ")
                .highlight_style(Style::new().bg(Color::Yellow).fg(Color::Black));

            let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("-"))
                .end_symbol(Some("-"));

            let mut scrollbar_state = ScrollbarState::new(items.len())
                .position(self.province_state.selected().unwrap_or_else(|| 0));

            frame.render_stateful_widget(list, bottom, &mut self.province_state);
            frame.render_stateful_widget(
                scrollbar,
                bottom.inner(Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut scrollbar_state,
            );

            if let Some(filter) = &self.filter {
                let paragraph = Paragraph::new(filter.clone()).block(
                    Block::bordered()
                        .title(" Filter ")
                        .title_bottom(" <Esc> Close ")
                        .padding(Padding::left(1)),
                );

                frame.render_widget(paragraph, top);
            }
        })?;

        Ok(())
    }

    pub fn render_all_kabkota(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        let (cols, rows) = terminal::size()?;
        terminal.draw(|frame| {
            let (x, y, width, height) = (
                (cols.saturating_sub(40)) / 2,
                (rows.saturating_sub(13)) / 2,
                40u16,
                13u16,
            );

            let rect = Rect::new(x, y, width, height);

            let [top, bottom] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Fill(1)])
                .areas(rect);

            let data = &self.kabkota_items.clone().unwrap_or_else(|| Vec::new());

            let items = match &self.kabkota_items_filter {
                Some(filtered_data) => &filtered_data
                    .iter()
                    .map(|f| ListItem::new(f.as_str()))
                    .collect(),
                None => &data
                    .iter()
                    .map(|f| ListItem::new(f.as_str()))
                    .collect::<Vec<ListItem>>(),
            };

            let list = List::new(items.clone())
                .block(
                    Block::bordered()
                        .title_bottom(" <q> Exit ")
                        .title_bottom("  Up ")
                        .title_bottom("  Down ")
                        .title_bottom("  Back ")
                        .title_top(" <f> filter ")
                        .padding(Padding::new(1, 2, 0, 0)),
                )
                .highlight_symbol(">> ")
                .highlight_style(Style::new().bg(Color::Yellow).fg(Color::Black));

            // FS
            let scrollbar = Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight);

            let mut scrollbar_state = ScrollbarState::new(items.len())
                .position(self.province_state.selected().unwrap_or_else(|| 0));

            frame.render_stateful_widget(list, bottom, &mut self.province_state);
            frame.render_stateful_widget(
                scrollbar,
                bottom.inner(Margin {
                    vertical: 1,
                    horizontal: 1,
                }),
                &mut scrollbar_state,
            );

            if let Some(filter) = &self.filter {
                let paragraph = Paragraph::new(filter.clone()).block(
                    Block::bordered()
                        .title(" Filter ")
                        .title_bottom(" <Esc> Close ")
                        .padding(Padding::left(1)),
                );

                frame.render_widget(paragraph, top);
            }
        })?;

        Ok(())
    }

    pub fn render_error(
        &self,
        terminal: &mut DefaultTerminal,
        error: &AppError,
    ) -> Result<(), AppError> {
        let (cols, rows) = terminal::size()?;
        terminal.draw(|frame| {
            let (x, y, width, height) = (
                (cols.saturating_sub(40)) / 2,
                (rows.saturating_sub(3)) / 2,
                40u16,
                3u16,
            );
            let rect = Rect::new(x, y, width, height);
            let paragraph = Paragraph::new(error.to_string())
                .block(Block::bordered())
                .centered();
            frame.render_widget(paragraph, rect);
        })?;

        Ok(())
    }

    fn handle_message(&self, frame: &mut Frame) -> Result<(), AppError> {
        if let Some(message) = &self.message {
            let (cols, rows) = terminal::size()?;
            let height = message.len() as u16 + 6;
            let (x, y, width, height) = (
                (cols.saturating_sub(height)) / 2,
                (rows.saturating_sub(3)) / 2,
                height,
                3u16,
            );

            let rect = Rect::new(x, y, width, height);
            let paragraph = Paragraph::new(message.as_str())
                .block(Block::bordered().bg(Color::Yellow).fg(Color::Black))
                .centered();
            frame.render_widget(paragraph, rect);
        }
        Ok(())
    }
}
