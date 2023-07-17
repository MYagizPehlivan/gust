use crossterm::{
    cursor, queue,
    style::{self, Color, Stylize},
    terminal, Result,
};
use std::io::{stdout, Write};

mod log;

pub struct Tui {
    log_panel: log::LogPanel,
    game: gust_core::Game,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            game: gust_core::Game::new(0),
            log_panel: log::LogPanel::new(),
        }
    }

    pub fn draw_main_panel(&self) -> Result<()> {
        queue!(stdout(), terminal::Clear(terminal::ClearType::All))?;

        let w = terminal::size()?.0;
        let h = terminal::size()?.1;

        draw_panel(0, 0, w, h).expect("Could not draw main panel");

        self.log_panel.draw(&self.game).expect("Could not draw log panel");

        stdout().flush()
    }
}

fn draw_panel(x: u16, y: u16, w: u16, h: u16) -> Result<()> {
    let panel_border_char: &str = "█";

    let bg_color = Color::Rgb { r: 10, g: 40, b: 50 };
    let border_color = Color::Rgb { r: 120, g: 170, b: 200 };

    if w < 2 || h < 2 {
        return Ok(());
    }

    let top_and_bottom_str = str::repeat(panel_border_char, w.into());
    let middle_str = String::new() + panel_border_char + " ".repeat((w - 2).into()).as_str() + panel_border_char;

    let top_and_bottom_stylized = top_and_bottom_str.as_str().with(border_color).on(bg_color);
    let middle_stylized = middle_str.as_str().with(border_color).on(bg_color);

    // Draw the top and bottom borders
    queue!(
        stdout(),
        cursor::MoveTo(x, y),
        style::PrintStyledContent(top_and_bottom_stylized),
        cursor::MoveTo(x, y + h - 1),
        style::PrintStyledContent(top_and_bottom_stylized),
    )?;

    // Draw the left and right vertical borders
    for draw_y in 1..h - 1 {
        queue!(stdout(), cursor::MoveTo(x, y + draw_y), style::PrintStyledContent(middle_stylized),)?;
    }

    Ok(())
}
