use std::{error::Error, io};
use arboard::Clipboard;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;

use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut clipboard = Clipboard::new().ok();

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                     app.on_quit();
                }
                KeyCode::Char('y') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                    if !app.show_help {
                        if let Some(index) = app.history_state.selected() {
                            if let Some(item) = app.history.get(index) {
                                if let Some(cb) = &mut clipboard {
                                    let _ = cb.set_text(&item.result);
                                }
                            }
                        }
                    }
                }
                KeyCode::Char('?') => app.toggle_help(),
                KeyCode::F(1) => app.toggle_help(),
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Backspace => app.on_backspace(),
                KeyCode::Enter => app.on_enter(),
                KeyCode::Up => app.select_previous(),
                KeyCode::Down => app.select_next(),
                KeyCode::Esc => {
                    if app.show_help {
                        app.show_help = false;
                    } else {
                        app.on_quit();
                    }
                }
                _ => {}
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
