use std::{
    f32::{consts::PI, INFINITY, NEG_INFINITY},
    io::stdout,
};

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
};
use glam::{Mat2, Mat3A, Mat4, Vec2, Vec3, Vec3A};

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
    view_matrix: Mat4,
    camera_matrix: Mat4,
    depth_buffer: ndarray::Array2<f32>,
}

const CAM_DISTANCE_TO_ORIGIN: f32 = 2.0;
const WORLD_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const CAMERA_UP_DIR: Vec3 = Vec3::new(0.0, 1.0, 0.0);

impl Log {
    pub fn new() -> Self {
        Self {
            index: 0,

            camera_position: Vec3A::new(0.0, 0.0, -CAM_DISTANCE_TO_ORIGIN),
            view_matrix: Mat4::ZERO,

            // Converts points from camera space to screen space.
            camera_matrix: Mat4::ZERO,

            depth_buffer: ndarray::Array2::<f32>::zeros((400, 400)),
        }
    }

    pub fn rotate(&mut self, direction: f32) {
        self.camera_position = Mat4::from_rotation_y(direction * PI / 20.0).transform_point3a(self.camera_position);
    }

    fn transform_world_vertex_to_camera_space(&self, v: Vec3A) -> Vec3A {
        self.view_matrix.transform_point3a(v)
    }

    fn project_camera_vertex_to_screen_space(&self, v: Vec3A, dims: &PanelDims) -> Vec3A {
        let mut result = self.camera_matrix.project_point3(v.into());
        result.x = (result.x + 1.0) / 2.0 * dims.w as f32 + dims.x as f32;
        result.y = dims.h as f32 - (result.y + 1.0) / 2.0 * dims.h as f32 + dims.y as f32;
        return result.into();
    }

    fn is_point_in_triangle(point: Vec3A, triangle_a: Vec3A, triangle_b: Vec3A, triangle_c: Vec3A) -> bool {
        let point_to_side_of_edge_ab = Mat2::from_cols((point - triangle_a).truncate(), (triangle_b - triangle_a).truncate()).determinant();

        if point_to_side_of_edge_ab == 0.0 {
            return true;
        }

        let point_to_side_of_edge_bc = Mat2::from_cols((point - triangle_b).truncate(), (triangle_c - triangle_b).truncate()).determinant();

        if point_to_side_of_edge_bc == 0.0 {
            return true;
        }

        let point_to_side_of_edge_ca = Mat2::from_cols((point - triangle_c).truncate(), (triangle_a - triangle_c).truncate()).determinant();

        if point_to_side_of_edge_ca == 0.0 {
            return true;
        }

        return point_to_side_of_edge_ab < 0.0 && point_to_side_of_edge_bc < 0.0 && point_to_side_of_edge_ca < 0.0;
    }

    pub fn draw(&mut self, dims: PanelDims, game: &gust_core::Game) -> Result<(), std::io::Error> {
        // Converts points from world space to camera space.
        self.view_matrix = Mat4::look_at_lh(self.camera_position.into(), WORLD_ORIGIN, CAMERA_UP_DIR);
        self.camera_matrix = Mat4::perspective_infinite_lh(1.5, (dims.w as f32) / (dims.h as f32) / 2.0, CAM_DISTANCE_TO_ORIGIN);

        let vertices = game.globe.data.raw_points();
        let triangle_indices = game.globe.data.get_all_indices();

        self.depth_buffer.fill(std::f32::INFINITY); // Clear the depth buffer

        // Rasterize each triangle
        for triangle in triangle_indices.chunks(3) {
            // Get the vertices of the triangle
            let v0 = vertices[triangle[0] as usize];
            let v1 = vertices[triangle[1] as usize];
            let v2 = vertices[triangle[2] as usize];

            // Transform the world space triangle into camera space
            let v0_camera = self.transform_world_vertex_to_camera_space(v0);
            let v1_camera = self.transform_world_vertex_to_camera_space(v1);
            let v2_camera = self.transform_world_vertex_to_camera_space(v2);

            // Project the camera space triangle into screen space
            let v0_screen = self.project_camera_vertex_to_screen_space(v0_camera, &dims);
            let v1_screen = self.project_camera_vertex_to_screen_space(v1_camera, &dims);
            let v2_screen = self.project_camera_vertex_to_screen_space(v2_camera, &dims);

            let x_start = *ndarray::array![dims.x, v0_screen.x as u16, v1_screen.x as u16, v2_screen.x as u16].iter().min().unwrap();
            let x_end = *ndarray::array![dims.x + dims.w, v0_screen.x as u16 + 1, v1_screen.x as u16 + 1, v2_screen.x as u16 + 1]
                .iter()
                .max()
                .unwrap();
            let y_start = *ndarray::array![dims.y, v0_screen.y as u16, v1_screen.y as u16, v2_screen.y as u16].iter().min().unwrap();
            let y_end = *ndarray::array![dims.y + dims.h, v0_screen.y as u16 + 1, v1_screen.y as u16 + 1, v2_screen.y as u16 + 1]
                .iter()
                .max()
                .unwrap();

            // y component of the "pixel" in absolute terminal coordinates
            for pixel_y in y_start..y_end {
                // x component of the "pixel" in absolute terminal coordinates
                for pixel_x in x_start..x_end {
                    let pixel_x_f32 = pixel_x as f32 + 0.5;
                    let pixel_y_f32 = pixel_y as f32 + 0.5;
                    if Log::is_point_in_triangle(Vec3A::new(pixel_x_f32, pixel_y_f32, 0.0), v0_screen, v1_screen, v2_screen) {
                        // TODO: do we need a more sophisticated depth?
                        let depth = (v0_screen.z + v1_screen.z + v2_screen.z) / 3.0;
                        let buffer_depth = self.depth_buffer[(pixel_x as usize, pixel_y as usize)];
                        if depth < self.depth_buffer[(pixel_x as usize, pixel_y as usize)] {
                            if buffer_depth.is_infinite() == false {
                                let pp = 1;
                            }
                            self.depth_buffer[(pixel_x as usize, pixel_y as usize)] = depth;

                            let styled = " ".on(style::Color::Rgb {
                                r: (v0.x * 255.0) as u8,
                                g: (v0.y * 255.0) as u8,
                                b: (v0.z * 255.0) as u8,
                            });
                            queue!(stdout(), cursor::MoveTo(pixel_x, pixel_y), style::PrintStyledContent(styled))?
                        }
                    }
                }
            }
        }

        // Set the BG color again because printing styled background resets it back to the terminal default
        queue!(stdout(), style::SetColors(style::Colors::new(BORDER_COLOR, BG_COLOR))).expect("Could not set TUI colors");
        Ok(())
    }
}

impl super::panel::Panel<Log> {
    pub fn draw(&mut self, dims: PanelDims, game: &gust_core::Game) -> Result<(), std::io::Error> {
        draw_panel(dims.x, dims.y, dims.w, dims.h).expect("Could not draw log panel");

        self.kind.draw(dims, game)
    }
}

pub type LogPanel = Panel<Log>;
