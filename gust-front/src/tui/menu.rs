use std::io::stdout;

use crossterm::{
    cursor,
    event::{Event, KeyCode},
    queue,
    style::{self},
};

use super::{
    draw_panel,
    panel::{Panel, PanelDims},
};

pub struct Menu {
    state: u8,
    options: Vec<String>,
}

impl Menu {
    pub fn new(options: Vec<String>) -> Self {
        Self { state: 0, options }
    }

    pub fn handle_key_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Up => {
                    if self.state == 0 {
                        self.state = self.options.len() as u8 - 1;
                    } else {
                        self.state -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.state == self.options.len() as u8 - 1 {
                        self.state = 0;
                    } else {
                        self.state += 1;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn draw(&self, dims: PanelDims, game: &gust_core::Game) -> Result<(), &str> {
        let option_count = self.options.len() as u16;

        if dims.h < 2 {
            return Ok(());
        }

        let menu_space = dims.h - 2; // Subtract 2 to ignore the panel borders
        if menu_space < option_count {
            return Ok(());
        }

        let y_step = menu_space / option_count;
        let options_space = option_count + (option_count - 1) * (y_step - 1);
        let margin = menu_space - options_space;

        let mut y = dims.y + 1 + margin / 2;

        for (index, opt) in self.options.iter().enumerate() {
            if index as u8 == self.state {
                queue!(
                    stdout(),
                    cursor::MoveTo(dims.x + 2, y),
                    style::SetAttribute(style::Attribute::Underlined),
                    style::Print(opt),
                    style::SetAttribute(style::Attribute::NoUnderline)
                )
                .expect("Couldn't draw menu option");
            } else {
                queue!(stdout(), cursor::MoveTo(dims.x + 2, y), style::Print(opt)).expect("Couldn't draw menu option");
            }
            y += y_step;
        }

        Ok(())
    }
}

impl Panel<Menu> {
    pub fn draw(&self, dims: PanelDims, game: &gust_core::Game) -> Result<(), &str> {
        draw_panel(dims.x, dims.y, dims.w, dims.h).expect("Couldn't draw menu panel");

        self.kind.draw(dims, game)
    }
}

pub type MenuPanel = Panel<Menu>;
