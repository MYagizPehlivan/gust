use crossterm::{
    cursor,
    event::{Event, KeyCode},
    execute, queue,
    style::{self, Color},
    terminal::{self, SetTitle},
    Result,
};
use gust_core::util::Fractionable;
use std::io::{stdout, Write};

use self::{
    log::{Log, LogPanel},
    menu::{Menu, MenuPanel},
    panel::PanelDims,
    status::{Status, StatusPanel},
};

mod log;
mod menu;
mod panel;
mod status;

const BG_COLOR: Color = Color::Rgb { r: 10, g: 40, b: 50 };
const BORDER_COLOR: Color = Color::Rgb { r: 120, g: 170, b: 200 };

pub struct Tui {
    game: gust_core::Game,
    log_panel: LogPanel,
    menu_panel: MenuPanel,
    status_panel: StatusPanel,
}

impl Tui {
    pub fn new() -> Self {
        execute!(stdout(), style::SetColors(style::Colors::new(BORDER_COLOR, BG_COLOR))).expect("Could not set TUI colors");

        Self {
            game: gust_core::Game::new(0),
            log_panel: LogPanel { kind: Log::new() },
            menu_panel: MenuPanel {
                kind: Menu::new(vec!["Move".to_string(), "Listen".to_string(), "Rest".to_string()]),
            },
            status_panel: StatusPanel { kind: Status {} },
        }
    }

    pub fn draw_main_panel(&mut self) -> Result<()> {
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

        let status_h = main_h.fraction(STATUS_PANEL_HEIGHT_FRACTION) + 1;
        let menu_h = main_h - main_h.fraction(STATUS_PANEL_HEIGHT_FRACTION);

        self.status_panel
            .draw(
                PanelDims {
                    x: main_w.fraction(STATUS_PANEL_WIDTH_FRACTION) - 1,
                    y: 0,
                    w: main_w - main_w.fraction(STATUS_PANEL_WIDTH_FRACTION) + 1,
                    h: status_h,
                },
                &self.game,
            )
            .expect("Could not draw status panel");

        self.menu_panel
            .draw(
                PanelDims {
                    x: main_w.fraction(LOG_PANEL_WIDTH_FRACTION) - 1,
                    y: main_h.fraction(STATUS_PANEL_HEIGHT_FRACTION),
                    w: main_w - main_w.fraction(LOG_PANEL_WIDTH_FRACTION) + 1,
                    h: menu_h,
                },
                &self.game,
            )
            .expect("Could not draw menu panel");

        stdout().flush()
    }

    pub fn handle_key_event(&mut self, event: crossterm::event::Event) {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('4') => {
                    self.log_panel.kind.rotate(-1.0);
                }
                KeyCode::Char('6') => {
                    self.log_panel.kind.rotate(1.0);
                }
                _ => {
                    self.menu_panel.kind.handle_key_event(event);
                }
            },
            _ => {}
        };
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
const STATUS_PANEL_HEIGHT_FRACTION: f32 = 0.50;
const STATUS_PANEL_WIDTH_FRACTION: f32 = 0.82;
