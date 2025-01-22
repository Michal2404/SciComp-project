use std::time::Instant;

use crate::rubiks::cubie::generate_scramble;
use crate::rubiks::cubie::generate_states;
use crate::rubiks::cubie::CubieCube;
use crate::rubiks::face::FaceCube;
use crate::rubiks::solver as sv;

use eframe::egui;
use eframe::egui::{Painter, Pos2, Rect, Vec2};

pub struct CubeVisualizer {
    pub cube: FaceCube,
}

impl CubeVisualizer {
    pub fn new(cube: FaceCube) -> Self {
        Self { cube }
    }

    pub fn update_cube(&mut self, cube: FaceCube) {
        self.cube = cube;
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

pub struct CubeVisualizerWithMoves {
    visualizer: CubeVisualizer,
    states: Vec<FaceCube>,
    current_index: usize,
    solution_string: String,
    user_scramble: String,
    error_message: String,
    use_ida: bool,
    solution_time: u128,
}

impl CubeVisualizerWithMoves {
    pub fn new(initial_cube: FaceCube, states: Vec<FaceCube>, solution: String) -> Self {
        Self {
            visualizer: CubeVisualizer::new(initial_cube),
            states,
            current_index: 0,
            solution_string: solution,
            user_scramble: String::new(),
            error_message: String::new(),
            use_ida: false,
            solution_time: 0,
        }
    }
}

impl eframe::App for CubeVisualizerWithMoves {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Ensure a consistent dark theme
        ctx.set_visuals(egui::Visuals::dark());
        egui::CentralPanel::default().show(ctx, |ui| {
            // Scramble input and generation
            ui.horizontal(|ui| {
                // "Load Tables" button
                if ui.button("Load Tables").clicked() {
                    self.user_scramble = "R U R' U' U R R' R'".to_string(); // Set the scramble to ""
                    self.error_message.clear(); // Clear previous error messages
                    self.current_index = 0;

                    let cubiecube = CubieCube::from_scramble(&self.user_scramble);

                    // Solve the predefined scramble
                    let solution = sv::solve(&self.user_scramble, 20, 2.0, true, false, None);

                    let trimmed_solution = solution
                        .rsplit_once('(')
                        .map_or(solution.clone(), |(before, _)| before.trim().to_string());
                    self.solution_string = trimmed_solution.trim().to_string();
                    self.states = generate_states(cubiecube, &trimmed_solution);
                    self.visualizer
                        .update_cube(cubiecube.to_facelet_cube().clone());
                    self.user_scramble = "".to_string();
                    let cubiecube = CubieCube::from_scramble(&self.user_scramble);

                    // Solve the predefined scramble
                    let solution = sv::solve(&self.user_scramble, 20, 2.0, true, false, None);

                    let trimmed_solution = solution
                        .rsplit_once('(')
                        .map_or(solution.clone(), |(before, _)| before.trim().to_string());
                    self.solution_string = trimmed_solution.trim().to_string();
                    self.states = generate_states(cubiecube, &trimmed_solution);
                    self.visualizer
                        .update_cube(cubiecube.to_facelet_cube().clone());
                }

                ui.label("Enter Scramble:");
                ui.add(egui::TextEdit::singleline(&mut self.user_scramble).desired_width(400.0));

                if ui.button("Generate Random Scramble").clicked() {
                    self.user_scramble = generate_scramble(20); // Generate a 20-move random scramble
                    self.error_message.clear(); // Clear any previous error messages
                }
            });

            // Add a toggle for the IDA* solver
            ui.checkbox(&mut self.use_ida, "Use IDA*");

            // Button to solve the scramble
            if ui.button("Solve Scramble").clicked() {
                let cubiecube = CubieCube::from_scramble(&self.user_scramble);
                self.error_message.clear(); // Clear previous error messages
                self.current_index = 0;

                let start_time = Instant::now();

                // Conditionally call the solver based on the toggle
                let solution = if self.use_ida {
                    sv::solve(&self.user_scramble, 20, 2.0, true, true, Some(10))
                // IDA* enabled
                } else {
                    sv::solve(&self.user_scramble, 20, 2.0, true, false, Some(10))
                    // IDA* disabled
                };

                self.solution_time = start_time.elapsed().as_millis();

                let trimmed_solution = solution
                    .rsplit_once('(')
                    .map_or(solution.clone(), |(before, _)| before.trim().to_string());
                self.solution_string = trimmed_solution.trim().to_string();
                self.states = generate_states(cubiecube, &trimmed_solution);
                self.visualizer
                    .update_cube(cubiecube.to_facelet_cube().clone());
            }

            // Display error messages
            if !self.error_message.is_empty() {
                ui.label(egui::RichText::new(&self.error_message).color(egui::Color32::RED));
            }

            // Display the solution and its length
            let solution_parts: Vec<_> = self
                .solution_string
                .split_whitespace()
                .enumerate()
                .map(|(i, move_str)| {
                    if i == self.current_index {
                        egui::RichText::new(move_str)
                            .color(egui::Color32::GREEN)
                            .size(32.0)
                    } else {
                        egui::RichText::new(move_str)
                            .color(egui::Color32::WHITE)
                            .size(32.0)
                    }
                })
                .collect();

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    for part in &solution_parts {
                        ui.label(part.clone());
                    }
                });

                let length = self.solution_string.split_whitespace().count();
                ui.label(
                    egui::RichText::new(format!("Solution length: {}", length))
                        .color(egui::Color32::WHITE)
                        .size(20.0),
                );
                ui.label(
                    egui::RichText::new(format!("Solution time: {} ms", self.solution_time))
                        .color(egui::Color32::WHITE)
                        .size(20.0),
                )
            });

            // Draw the cube
            let available_size = ui.available_size();
            let square_size = (available_size.x.min(available_size.y) / 12.0).max(20.0);
            let cube_size = egui::Vec2::new(9.0 * square_size, 6.0 * square_size);

            if available_size.x < cube_size.x || available_size.y < cube_size.y {
                ui.label("Not enough space to render the cube! Resize the window.");
            } else {
                let top_left = (available_size - cube_size) / 2.0;
                let top_left = ui.min_rect().min + top_left;
                draw_face_cube(&self.visualizer.cube, ui.painter(), top_left, square_size);
            }

            ui.add_space(20.0); // Add spacing between the cube and the buttons

            // Add larger buttons for "Next Move" and "Reset"
            if ui
                .add(egui::Button::new("Next Move").min_size(egui::vec2(150.0, 50.0)))
                .clicked()
                && self.current_index < self.states.len()
            {
                self.visualizer
                    .update_cube(self.states[self.current_index].clone());
                self.current_index += 1;
            }

            if ui
                .add(egui::Button::new("Reset").min_size(egui::vec2(150.0, 50.0)))
                .clicked()
            {
                let cubiecube = CubieCube::from_scramble(&self.user_scramble);
                self.current_index = 0;
                self.visualizer
                    .update_cube(cubiecube.to_facelet_cube().clone());
            }
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
