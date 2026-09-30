use ratatui::{
    Frame,
    crossterm::terminal,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    models::View,
    tui::{App, AppState},
};

impl<T: View> App<T> {
    pub fn render_no_cache(&self, frame: &mut Frame, area: Rect) {
        let (cols, rows) = terminal::size().unwrap();
        let (mut width, mut height) = (55u16, 3u16);

        let (mut x, mut y) = (
            (cols.saturating_sub(width)) / 2,
            (rows.saturating_sub(height)) / 2,
        );

        let text = "Jalankan shlt -d terlebih dahulu di terminal anda";
        let mut message = vec![Line::from(text)];

        if width >= cols {
            width = (width / 3) + 3;
            height = height * 2;
            message = vec![
                Line::from(&text[0..16]),
                Line::from(&text[16..32]),
                Line::from(&text[32..]),
            ];
            x = (cols.saturating_sub(width)) / 2;
            y = (rows.saturating_sub(height)) / 2;
        }

        let rect = Rect::new(x, y, width, height);
        frame.render_widget(
            Paragraph::new(message).block(Block::bordered()).centered(),
            rect,
        );
    }
    pub fn render_view(&self, frame: &mut Frame, area: Rect) {
        let (cols, rows) = terminal::size().unwrap();
        let (x, y, width, height) = ((cols - 30) / 2, (rows - 8) / 2, 30u16, 8u16);

        let rect = Rect::new(x, y, width, height);
        let top = Rect::new(x, y - 3, width, 3);
        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .areas(rect);

        match &self.state {
            AppState::View(jadwal) => {
                frame.render_widget(
                    Paragraph::new(jadwal.get_kabkota())
                        .centered()
                        .block(Block::bordered()),
                    top,
                );

                frame.render_widget(
                    Paragraph::new(vec![
                        Line::from("Subuh"),
                        Line::from("Dzuhur"),
                        Line::from("Ashar"),
                        Line::from("Maghrib"),
                        Line::from("Isya"),
                    ])
                    .block(Block::bordered())
                    .centered(),
                    left,
                );
                frame.render_widget(
                    Paragraph::new(vec![
                        Line::from(jadwal.get_subuh()),
                        Line::from(jadwal.get_dzuhur()),
                        Line::from(jadwal.get_ashar()),
                        Line::from(jadwal.get_maghrib()),
                        Line::from(jadwal.get_isya()),
                    ])
                    .block(Block::bordered())
                    .centered(),
                    right,
                );
            }
            _ => {}
        }
    }
}
