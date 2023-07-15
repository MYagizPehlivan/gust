use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, terminal, Result,
};
use std::io::stdout;

mod display;

fn main() -> Result<()> {
    let (orig_w, orig_h) = terminal::size()?;

    // Resize terminal and draw window
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, terminal::SetSize(128, 48), cursor::Hide)?;

    let game = gust_core::Game { time_in_seconds: 0 };

    display::draw_main_window(&game)?;

    loop {
        match read()? {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
            Event::Resize(_new_w, _new_h) => {
                display::draw_main_window(&game)?;
            }
            _ => {}
        };
    }

    // Clean up and exit
    execute!(stdout(), terminal::LeaveAlternateScreen, terminal::SetSize(orig_w, orig_h), cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
