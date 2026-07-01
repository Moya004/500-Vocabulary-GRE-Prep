use ratatui::{
    Frame,
    layout::{self, Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Paragraph},
};

use crate::types::{CurrentScreen, PresentingScreen};

pub fn render_footer(frame: &mut Frame, area: Rect, current_screen: CurrentScreen) {
    match current_screen {
        CurrentScreen::Welcome => welcome_footer(frame, area),
        CurrentScreen::Presenting(PresentingScreen::Front) => {
            presenting_footer(frame, area, PresentingScreen::Front)
        }
        CurrentScreen::Presenting(PresentingScreen::Back) => {
            presenting_footer(frame, area, PresentingScreen::Back);
        }
        _ => {}
    }
}

pub fn welcome_footer(frame: &mut Frame, area: Rect) {
    let footer_layout = Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(area);

    let quit_block = Paragraph::new("q: Quit")
        .block(Block::default().style(Style::default()))
        .centered();

    let search_block = Paragraph::new("s: Search Word")
        .block(Block::default().style(Style::default()))
        .centered();

    let next_block = Paragraph::new("n: Next Word")
        .block(Block::default().style(Style::default()))
        .centered();

    frame.render_widget(quit_block, footer_layout[0]);
    frame.render_widget(search_block, footer_layout[1]);
    frame.render_widget(next_block, footer_layout[2]);
}

pub fn presenting_footer(frame: &mut Frame, area: Rect, screen: PresentingScreen) {
    let footer_layout = Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let quit_block = Paragraph::new("q: Quit")
        .block(Block::default().style(Style::default()))
        .centered();

    let search_block = Paragraph::new("s: Search Word")
        .block(Block::default().style(Style::default()))
        .centered();

    let flip_block = Paragraph::new(match screen {
        PresentingScreen::Front => "f: Flip to back",
        PresentingScreen::Back => "f: Flip to front",
    })
    .block(Block::default().style(Style::default()))
    .centered();

    let next_block = Paragraph::new("n: Next Word")
        .block(Block::default().style(Style::default()))
        .centered();

    frame.render_widget(quit_block, footer_layout[0]);
    frame.render_widget(search_block, footer_layout[1]);
    frame.render_widget(flip_block, footer_layout[2]);
    frame.render_widget(next_block, footer_layout[3]);
}
