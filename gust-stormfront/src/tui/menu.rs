use std::io::stdout;

use crossterm::{cursor, queue, style};

use super::{
    draw_panel,
    panel::{Panel, PanelDims},
};

pub struct Menu {
    state: u8,
}

impl Menu {
    pub fn new() -> Self {
        Self { state: 0 }
    }
}

impl Panel<Menu> {
    pub fn draw(&self, dims: PanelDims, game: &gust_core::Game) -> Result<(), std::io::Error> {
        draw_panel(dims.x, dims.y, dims.w, dims.h)?;

        queue!(stdout(), cursor::MoveTo(dims.x + 1, dims.y + 1), style::Print(self.kind.state.to_string()))
    }
}

pub type MenuPanel = Panel<Menu>;
