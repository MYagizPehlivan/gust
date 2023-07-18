use super::{
    draw_panel,
    panel::{Panel, PanelDims},
};

pub struct Log {
    /// Offset from the end of the log. Hence, 0 means we're displaying
    /// the last line of the log at the bottom of the panel.
    index: u64,
}

impl Log {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn draw(&self, game: &gust_core::Game) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl super::panel::Panel<Log> {
    pub fn draw(&self, dims: PanelDims, game: &gust_core::Game) -> Result<(), std::io::Error> {
        draw_panel(dims.x, dims.y, dims.w, dims.h)
    }
}

pub type LogPanel = Panel<Log>;
