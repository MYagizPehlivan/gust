use crossterm::{
    cursor, execute, queue,
    style::{self, Color},
    terminal, Result,
};
use gust_core::util::Fractionable;
use std::io::{stdout, Write};

use self::{
    log::Log,
    menu::Menu,
    panel::{Panel, PanelDims},
};

mod log;
mod menu;
mod panel;

const BG_COLOR: Color = Color::Rgb { r: 10, g: 40, b: 50 };
const BORDER_COLOR: Color = Color::Rgb { r: 120, g: 170, b: 200 };

pub struct Tui {
    game: gust_core::Game,
    log_panel: Panel<Log>,
    menu_panel: Panel<Menu>,
}

impl Tui {
    pub fn new() -> Self {
        execute!(stdout(), style::SetColors(style::Colors::new(BORDER_COLOR, BG_COLOR))).expect("Could not set TUI colors");

        Self {
            game: gust_core::Game::new(0),
            log_panel: log::LogPanel { kind: Log::new() },
            menu_panel: menu::MenuPanel { kind: Menu::new() },
        }
    }

    pub fn draw_main_panel(&self) -> Result<()> {
        queue!(stdout(), terminal::Clear(terminal::ClearType::All))?;

        let main_w = terminal::size()?.0;
        let main_h = terminal::size()?.1;

        draw_panel(0, 0, main_w, main_h).expect("Could not draw main panel");

        self.log_panel
            .draw(
                PanelDims {
                    x: 0,
                    y: 0,
                    w: main_w.fraction(LOG_PANEL_WIDTH_FRACTION),
                    h: main_h,
                },
                &self.game,
            )
            .expect("Could not draw log panel");

        self.menu_panel
            .draw(
                PanelDims {
                    x: main_w.fraction(LOG_PANEL_WIDTH_FRACTION) - 1,
                    y: main_h.fraction(MENU_PANEL_HEIGHT_FRACTION),
                    w: main_w - main_w.fraction(LOG_PANEL_WIDTH_FRACTION) + 1,
                    h: main_h - main_h.fraction(MENU_PANEL_HEIGHT_FRACTION),
                },
                &self.game,
            )
            .expect("Could not draw menu panel");

        stdout().flush()
    }
}

fn draw_panel(x: u16, y: u16, w: u16, h: u16) -> Result<()> {
    let panel_border_char: &str = "█";

    if w < 2 || h < 2 {
        return Ok(());
    }

    let top_and_bottom_str = str::repeat(panel_border_char, w.into());
    let middle_str = String::new() + panel_border_char + " ".repeat((w - 2).into()).as_str() + panel_border_char;

    // Draw the top and bottom borders
    queue!(
        stdout(),
        cursor::MoveTo(x, y),
        style::Print(top_and_bottom_str.as_str()),
        cursor::MoveTo(x, y + h - 1),
        style::Print(top_and_bottom_str.as_str()),
    )?;

    // Draw the left and right vertical borders
    for draw_y in 1..h - 1 {
        queue!(stdout(), cursor::MoveTo(x, y + draw_y), style::Print(middle_str.as_str()),)?;
    }

    Ok(())
}

const LOG_PANEL_WIDTH_FRACTION: f32 = 0.64;
const MENU_PANEL_HEIGHT_FRACTION: f32 = 0.50;
const STATUS_PANEL_WIDTH_FRACTION: f32 = 0.82;
