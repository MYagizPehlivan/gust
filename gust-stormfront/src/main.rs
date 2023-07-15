use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, terminal, Result,
};
use std::io::stdout;

mod windows;

fn main() -> Result<()> {
    let (orig_w, orig_h) = terminal::size()?;

    // Resize terminal and draw window
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, terminal::SetSize(128, 48), cursor::Hide)?;

    let gui = windows::Gui::new();

    gui.draw_main_window()?;

    loop {
        match read()? {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
            Event::Resize(_new_w, _new_h) => {
                gui.draw_main_window()?;
            }
            _ => {}
        };
    }

    // Clean up and exit
    execute!(stdout(), terminal::LeaveAlternateScreen, terminal::SetSize(orig_w, orig_h), cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
