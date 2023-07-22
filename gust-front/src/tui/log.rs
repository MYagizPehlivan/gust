use std::io::stdout;

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
};
use glam::{Mat3A, Vec3A};

use crate::tui::{BG_COLOR, BORDER_COLOR};

use super::{
    draw_panel,
    panel::{Panel, PanelDims},
};

pub struct Log {
    /// Offset from the end of the log. Hence, 0 means we're displaying
    /// the last line of the log at the bottom of the panel.
    index: u64,

    pub camera_position: Vec3A,
    pub camera_orientation: Vec3A,
    pub display_surface_offset: Vec3A,
}

const CAM_DISTANCE_TO_ORIGIN: f32 = -2.0;
const MAX_VERTEX_DISTANCE_SQUARED_TO_RENDER: f32 = 1.0 + CAM_DISTANCE_TO_ORIGIN * CAM_DISTANCE_TO_ORIGIN; // 1.0 is the radius of the globe.

impl Log {
    pub fn new() -> Self {
        Self {
            index: 0,

            camera_position: Vec3A::new(0.0, 0.0, CAM_DISTANCE_TO_ORIGIN),
            camera_orientation: Vec3A::new(0.0, 0.0, 0.0),
            display_surface_offset: Vec3A::new(0.0, 0.0, 1.0),
        }
    }

    pub fn draw(&self, dims: PanelDims, game: &gust_core::Game) -> Result<(), std::io::Error> {
        // TODO: render in a way that doesn't leave gaps in the frame

        let fitting_size = dims.w.min(dims.h);
        for vertex in game.globe.data.raw_points() {
            // Don't render vertices on the far side of the globe
            if vertex.distance_squared(self.camera_position) > MAX_VERTEX_DISTANCE_SQUARED_TO_RENDER {
                continue;
            }

            let x_transform = Mat3A::from_rotation_x(self.camera_orientation.x);
            let y_transform = Mat3A::from_rotation_y(self.camera_orientation.y);
            let z_transform = Mat3A::from_rotation_z(self.camera_orientation.z);
            let camera_transformed_vertex = x_transform * y_transform * z_transform * (*vertex - self.camera_position);

            let x = self.display_surface_offset.z / camera_transformed_vertex.z * camera_transformed_vertex.x + self.display_surface_offset.x;
            let y = self.display_surface_offset.z / camera_transformed_vertex.z * camera_transformed_vertex.y + self.display_surface_offset.y;

            let panel_x = ((x + 1.0) / 2.0 * dims.w as f32) as u16 + dims.x;
            let panel_y = ((y + 1.0) / 2.0 * dims.h as f32) as u16 + dims.y;

            if panel_x > dims.x && panel_x < dims.x + dims.w && panel_y > dims.y && panel_y < dims.y + dims.h {
                let styled = " ".on(style::Color::Rgb {
                    r: (vertex.x * 255.0) as u8,
                    g: (vertex.y * 255.0) as u8,
                    b: (vertex.z * 255.0) as u8,
                });
                queue!(stdout(), cursor::MoveTo(panel_x, panel_y), style::PrintStyledContent(styled))?
            }
        }

        // Set the BG color again because printing styled background resets it back to the terminal default
        queue!(stdout(), style::SetColors(style::Colors::new(BORDER_COLOR, BG_COLOR))).expect("Could not set TUI colors");
        Ok(())
    }
}

impl super::panel::Panel<Log> {
    pub fn draw(&self, dims: PanelDims, game: &gust_core::Game) -> Result<(), std::io::Error> {
        draw_panel(dims.x, dims.y, dims.w, dims.h).expect("Could not draw log panel");

        self.kind.draw(dims, game)
    }
}

pub type LogPanel = Panel<Log>;
