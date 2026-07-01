mod db;
mod logic;
mod types;
mod ui;
use std::{error::Error, io, thread, time::Duration};

use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use types::App;

use crate::{
    logic::{
        commands::{Command, NextRandomWordCommand, SaveNewWordCommand},
        handlers::NextRandomWordCommandHandler,
    },
    types::NewWordEntry,
};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    // let mut w = NewWordEntry::new();
    // w.entry_input = String::from("Aberrant");
    // w.class_input = String::from("adj");
    // w.pronunciation_input = String::from("uh `ber` unt");
    // w.definition_input = String::from("deviating from what is nolmal or expected");
    // w.example_input = String::from(
    //     "Since he had been a steady, cheerful worker for many years, his fellow postal workers did not expect his `aberrant` burst of rage",
    // );

    // let cmd = SaveNewWordCommand::new(w);
    //
    // terminal.draw(|_| print!("\r\n Save command output: {:?}", cmd.execute()))?;

    // for i in 0..10 {
    //     terminal.draw(|_| print!("\r\nIt's running! {}", i))?;
    //     terminal.draw(|_| {
    //         print!(
    //             "\r\nNext random word output: {:?}",
    //             NextRandomWordCommand {}.execute()
    //         )
    //     })?;
    //     thread::sleep(Duration::from_secs(5));
    // }
    loop {
        let nxt_rnd_word_cmd_handler = NextRandomWordCommandHandler {};
        terminal.draw(|f| ui::app::ui_render(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                types::CurrentScreen::Welcome => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }

                    KeyCode::Char('n') => {
                        app.current_screen =
                            types::CurrentScreen::Presenting(types::PresentingScreen::Front);
                        app.current_word =
                            nxt_rnd_word_cmd_handler.execute_command(NextRandomWordCommand {});
                    }
                    _ => {}
                },

                types::CurrentScreen::Presenting(_) => match key.code {
                    KeyCode::Char('h') => {
                        app.current_screen = types::CurrentScreen::Welcome;
                    }

                    KeyCode::Char('f') => match app.current_screen {
                        types::CurrentScreen::Presenting(types::PresentingScreen::Back) => {
                            app.current_screen =
                                types::CurrentScreen::Presenting(types::PresentingScreen::Front);
                        }
                        types::CurrentScreen::Presenting(types::PresentingScreen::Front) => {
                            app.current_screen =
                                types::CurrentScreen::Presenting(types::PresentingScreen::Back);
                        }
                        _ => {}
                    },

                    KeyCode::Char('n') => {
                        app.current_word =
                            nxt_rnd_word_cmd_handler.execute_command(NextRandomWordCommand {});
                        app.current_screen =
                            types::CurrentScreen::Presenting(types::PresentingScreen::Front);
                    }

                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(true)
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut app = App::new();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let _ = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}
