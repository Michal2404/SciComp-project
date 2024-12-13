use crate::rubiks::cube::RubiksCube;
use eframe::egui;
use eframe::egui::{Painter, Pos2, Rect, Vec2};

pub struct MyApp {
    pub cube: RubiksCube,
}

impl MyApp {
    pub fn new(cube: RubiksCube) -> Self {
        Self { cube }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let square_size = (available_size.x.min(available_size.y) / 12.0).max(20.0); // Adjust square size dynamically

            let cube_size = egui::Vec2::new(9.0 * square_size, 6.0 * square_size);
            if available_size.x < cube_size.x || available_size.y < cube_size.y {
                ui.label("Not enough space to render the cube! Resize the window.");
                return;
            }

            let top_left = (available_size - cube_size) / 2.0;
            let top_left = ui.min_rect().min + top_left;

            draw_cube(&self.cube, ui.painter(), top_left, square_size);
        });
    }
}

pub fn draw_cube(cube: &RubiksCube, painter: &Painter, top_left: Pos2, square_size: f32) {
    let face_positions = [
        (0, -1), // Top
        (0, 1),  // Bottom
        (0, 0),  // Front
        (2, 0),  // Back
        (1, 0),  // Right
        (-1, 0), // Left
    ];

    for (face_idx, &(dx, dy)) in face_positions.iter().enumerate() {
        let face = &cube.faces[face_idx];
        for y in 0..3 {
            for x in 0..3 {
                let color = face[y * 3 + x].to_color32();
                let rect = Rect::from_min_size(
                    top_left
                        + Vec2::new((dx * 3 + x as i32) as f32, (dy * 3 + y as i32) as f32)
                            * square_size,
                    Vec2::splat(square_size),
                );
                painter.rect_filled(rect, 0.0, color);
            }
        }
    }
}
