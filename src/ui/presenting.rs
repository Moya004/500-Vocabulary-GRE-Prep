use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    text::Text,
};
use tui_big_text::BigText;

use crate::{
    types::{PresentingScreen, Word},
    ui::footer::render_footer,
};

pub fn render_presenting_screen(frame: &mut Frame, screen: PresentingScreen, to_display: Word) {
    let layout = Layout::default()
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Percentage(10),
        ])
        .split(frame.area());

    let mut main_lines = vec![];
    let mut scnd_lines = vec![];
    let mut main_text = BigText::builder();
    let mut scnd_text = BigText::builder();
    let mut more_text = Text::default();

    match screen {
        PresentingScreen::Front => {
            main_lines.push(to_display.entry.clone().red().into());
            scnd_text.pixel_size(tui_big_text::PixelSize::Quadrant);
            scnd_lines.push(to_display.class.clone().light_magenta().into());
            more_text.push_line(to_display.pronunciation.clone());
        }

        PresentingScreen::Back => {
            main_lines.push(to_display.definition.clone().light_blue().into());
            main_text.pixel_size(tui_big_text::PixelSize::Octant);
            more_text.push_line(to_display.example.clone());
        }
    }

    main_text.lines(main_lines).centered();
    scnd_text.lines(scnd_lines).centered();
    frame.render_widget(main_text.build(), layout[1]);
    frame.render_widget(scnd_text.build(), layout[2]);
    frame.render_widget(
        &more_text,
        layout[3].centered(
            Constraint::Length(more_text.width() as u16),
            Constraint::Length(1),
        ),
    );
    render_footer(
        frame,
        layout[4],
        crate::types::CurrentScreen::Presenting(screen),
    );
}
