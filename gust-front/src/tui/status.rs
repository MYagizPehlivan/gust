use std::io::stdout;

use crossterm::{cursor, queue, style};

use super::{
    draw_panel,
    panel::{Panel, PanelDims},
};

pub struct Status {}

impl Status {}

impl Panel<Status> {
    pub fn draw(&self, dims: PanelDims, game: &gust_core::Game) -> Result<(), std::io::Error> {
        draw_panel(dims.x, dims.y, dims.w, dims.h)?;

        queue!(
            stdout(),
            cursor::MoveTo(dims.x + 2, dims.y + 2),
            style::Print(game.player.name.to_string()),
            cursor::MoveTo(dims.x + 2, dims.y + 4),
            style::Print(&game.player.position)
        )
    }
}

pub type StatusPanel = Panel<Status>;
