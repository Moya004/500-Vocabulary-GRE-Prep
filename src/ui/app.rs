use ratatui::Frame;

use crate::{
    types::{App, PresentingScreen},
    ui::{presenting::render_presenting_screen, welcome::render_welcome_screen},
};

pub fn ui_render(frame: &mut Frame, app: &App) {
    match app.current_screen {
        crate::types::CurrentScreen::Welcome => {
            render_welcome_screen(frame);
        }

        crate::types::CurrentScreen::Presenting(PresentingScreen::Front) => {
            let word = app.current_word.clone().unwrap();
            render_presenting_screen(frame, PresentingScreen::Front, word);
        }

        crate::types::CurrentScreen::Presenting(PresentingScreen::Back) => {
            let word = app.current_word.clone().unwrap();
            render_presenting_screen(frame, PresentingScreen::Back, word);
        }

        _ => {}
    }
}
