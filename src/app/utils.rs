use ratatui::{
    Frame,
    style::{Color, Stylize},
    widgets::Paragraph,
};

pub fn background_color(frame: &mut Frame, color: Color) {
    let background = Paragraph::new("").bg(color);
    frame.render_widget(background, frame.area());
}
