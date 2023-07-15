use crossterm::terminal;
use gust_core::util::Fractionable;

use super::draw_window;

pub struct LogWindow {
    /// Offset from the end of the log. Hence, 0 means we're displaying
    /// the last line of the log at the bottom of the window.
    index: u64,
}

impl LogWindow {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn draw(&self, game: &gust_core::Game) -> Result<(), std::io::Error> {
        let game_w = terminal::size()?.0;
        let game_h = terminal::size()?.1;

        draw_window(0, 0, game_w.fraction(0.64), game_h)
    }
}
