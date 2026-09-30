#![allow(unused)]
use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        terminal,
    },
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::{
    app,
    models::{JadwalShalat, View},
};

mod render;

pub struct App<T: View> {
    state: AppState<T>,
    exit: bool,
}

pub enum AppState<T: View> {
    View(T),
    NoCache,
    Test,
}

impl<T: View> App<T> {
    pub fn new(state: AppState<T>) -> Self {
        Self { state, exit: false }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        match self.state {
            AppState::View(_) => self.render_view(frame, area),
            AppState::NoCache => self.render_no_cache(frame, area),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                self.handle_key_event(key);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            _ => {}
        }
    }
}

#[test]
fn test_tui() {
    ratatui::run(|terminal| {
        // let app_state = AppState::View(JadwalShalat::default());
        let app_state = AppState::<()>::NoCache;
        let mut app = App {
            exit: false,
            state: app_state,
        };
        app.run(terminal).unwrap()
    });

    println!("{:?}", terminal::size().unwrap());
}

#[test]
fn test_lmao() {
    let message = "Jalankan shlt -d terlebih dahulu di terminal anda";
    let msg1 = &message[0..16];
    let msg2 = &message[16..32];
    let msg3 = &message[32..];

    println!("{}", msg1.trim());
    println!("{}", msg2.trim());
    println!("{}", msg3.trim());
}
