use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, terminal, Result,
};
use std::io::stdout;

mod tui;

fn main() -> Result<()> {
    let (orig_w, orig_h) = terminal::size()?;

    // Resize terminal and draw panel
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, terminal::SetSize(128, 48), cursor::Hide)?;

    let mut tui = tui::Tui::new();

    tui.draw_main_panel()?;

    'input_loop: loop {
        let event = read()?;
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') => {
                    break 'input_loop;
                }
                _ => {
                    tui.handle_key_event(event);
                    tui.draw_main_panel()?;
                }
            },
            Event::Resize(_new_w, _new_h) => {
                tui.draw_main_panel()?;
            }
            _ => {}
        };
    }

    // Clean up and exit
    execute!(stdout(), terminal::LeaveAlternateScreen, terminal::SetSize(orig_w, orig_h), cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
