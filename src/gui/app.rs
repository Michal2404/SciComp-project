use crate::rubiks::cubie::CubieCube;
use crate::rubiks::enums::Color;
use crate::rubiks::face::FaceCube;
use eframe::egui;
use eframe::egui::{Painter, Pos2, Rect, Vec2};

pub struct CubeVisualizer {
    pub cube: FaceCube,
}

impl CubeVisualizer {
    pub fn new(cube: FaceCube) -> Self {
        Self { cube }
    }
}

impl eframe::App for CubeVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let square_size = (available_size.x.min(available_size.y) / 12.0).max(20.0);

            let cube_size = egui::Vec2::new(9.0 * square_size, 6.0 * square_size);
            if available_size.x < cube_size.x || available_size.y < cube_size.y {
                ui.label("Not enough space to render the cube! Resize the window.");
                return;
            }

            let top_left = (available_size - cube_size) / 2.0;
            let top_left = ui.min_rect().min + top_left;

            draw_face_cube(&self.cube, ui.painter(), top_left, square_size);
        });
    }
}

fn draw_face_cube(cube: &FaceCube, painter: &Painter, top_left: Pos2, square_size: f32) {
    let face_positions = [
        (3, 0), // Top (U)
        (6, 3), // Bottom (D)  6,3
        (3, 3), // Front (F)
        (3, 6), // Left (L)    3,6
        (0, 3), // Right (R)   0,3
        (9, 3), // Back (B)
    ];

    for (face_idx, &(dx, dy)) in face_positions.iter().enumerate() {
        for y in 0..3 {
            for x in 0..3 {
                let color = cube.f[face_idx * 9 + y * 3 + x];
                let rect = Rect::from_min_size(
                    top_left + Vec2::new((dx + x) as f32, (dy + y) as f32) * square_size,
                    Vec2::splat(square_size),
                );
                painter.rect_filled(rect, 0.0, color.to_color32());
            }
        }
    }
}
