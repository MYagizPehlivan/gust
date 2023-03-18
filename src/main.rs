use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, style, terminal, Result,
};
use std::io::stdout;

fn draw_window(w: u16, h: u16) -> Result<()> {
    // let window_border_char: &str = "█";
    let window_border_char: &str = "#";

    if w < 2 || h < 2 {
        return Ok(());
    }

    // Clear the window before starting drawing
    execute!(
        stdout(),
        terminal::SetTitle(format!("w: {}, h: {}", terminal::size()?.0, terminal::size()?.1)),
        terminal::Clear(terminal::ClearType::All)
    )?;

    // Draw the top and bottom borders
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        style::Print(str::repeat(window_border_char, w.into())),
        cursor::MoveTo(0, h - 1),
        style::Print(str::repeat(window_border_char, w.into())),
    )?;

    // Draw the left and right vertical borders
    for y in 1..h - 1 {
        execute!(
            stdout(),
            cursor::MoveTo(0, y),
            style::Print(window_border_char),
            cursor::MoveTo(w - 1, y),
            style::Print(window_border_char),
        )?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let (orig_w, orig_h) = terminal::size()?;

    // Resize terminal and draw window
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, terminal::SetSize(40, 40), cursor::Hide)?;

    draw_window(terminal::size()?.0, terminal::size()?.1)?;

    loop {
        match read()? {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
            Event::Resize(new_w, new_h) => {
                draw_window(new_w, new_h)?;
            }
            _ => {}
        };
    }

    // Clean up and exit
    execute!(stdout(), terminal::LeaveAlternateScreen, terminal::SetSize(orig_w, orig_h))?;
    terminal::disable_raw_mode()?;
    Ok(())
}
