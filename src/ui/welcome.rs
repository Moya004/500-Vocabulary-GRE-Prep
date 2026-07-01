use ratatui::{
    Frame,
    layout::{self, Constraint, Layout},
    style::{Color, Stylize},
};
use tui_big_text::BigText;

use crate::ui::footer::render_footer;

pub fn render_welcome_screen(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(70),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ])
        .split(frame.area());

    let title = BigText::builder()
        .lines(vec![
            "Welcome".fg(Color::Rgb(17, 56, 204)).into(),
            "500 Word GRE Prep".fg(Color::Rgb(204, 17, 58)).into(),
            "Dictionary".fg(Color::Rgb(58, 204, 17)).into(),
        ])
        .centered()
        .build();

    let sub_title = BigText::builder()
        .lines(vec!["by: Mario Martinez".white().into()])
        .centered()
        .pixel_size(tui_big_text::PixelSize::Octant)
        .build();
    frame.render_widget(title, chunks[0]);
    frame.render_widget(sub_title, chunks[1]);
    render_footer(frame, chunks[2], crate::types::CurrentScreen::Welcome);
}
