// // use crate::rubiks::cube::RubiksCube;
// // use eframe::egui;
// // use eframe::egui::{Painter, Pos2, Rect, Vec2};

// // pub struct MyApp {
// //     pub cube: RubiksCube,
// // }

// // impl MyApp {
// //     pub fn new(cube: RubiksCube) -> Self {
// //         Self { cube }
// //     }
// // }

// // impl eframe::App for MyApp {
// //     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
// //         egui::CentralPanel::default().show(ctx, |ui| {
// //             let available_size = ui.available_size();
// //             let square_size = (available_size.x.min(available_size.y) / 12.0).max(20.0); // Adjust square size dynamically

// //             let cube_size = egui::Vec2::new(9.0 * square_size, 6.0 * square_size);
// //             if available_size.x < cube_size.x || available_size.y < cube_size.y {
// //                 ui.label("Not enough space to render the cube! Resize the window.");
// //                 return;
// //             }

// //             let top_left = (available_size - cube_size) / 2.0;
// //             let top_left = ui.min_rect().min + top_left;

// //             draw_cube(&self.cube, ui.painter(), top_left, square_size);
// //         });
// //     }
// // }

// // pub fn draw_cube(cube: &RubiksCube, painter: &Painter, top_left: Pos2, square_size: f32) {
// //     let face_positions = [
// //         (0, -1), // Top
// //         (0, 1),  // Bottom
// //         (0, 0),  // Front
// //         (2, 0),  // Back
// //         (1, 0),  // Right
// //         (-1, 0), // Left
// //     ];

// //     for (face_idx, &(dx, dy)) in face_positions.iter().enumerate() {
// //         let face = &cube.faces[face_idx];
// //         for y in 0..3 {
// //             for x in 0..3 {
// //                 let color = face[y * 3 + x].to_color32();
// //                 let rect = Rect::from_min_size(
// //                     top_left
// //                         + Vec2::new((dx * 3 + x as i32) as f32, (dy * 3 + y as i32) as f32)
// //                             * square_size,
// //                     Vec2::splat(square_size),
// //                 );
// //                 painter.rect_filled(rect, 0.0, color);
// //             }
// //         }
// //     }
// // }


// // This file creates the 3d animation of rubiks cube GUI
// use bevy::prelude::*;
// use crate::rubiks::cube::RubiksCube;

// #[derive(Component)]
// struct Cube;


// pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
//     // Camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });

//     // Light
//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     });

//     // Cube with colored faces
//     let colors = [
//         Color::RED, Color::GREEN, Color::BLUE,
//         Color::YELLOW, Color::WHITE, Color::ORANGE,
//     ];

//     for (i, &color) in colors.iter().enumerate() {
//         let rotation = match i {
//             0 => Quat::from_rotation_y(0.0),         // Front
//             1 => Quat::from_rotation_y(std::f32::consts::PI), // Back
//             2 => Quat::from_rotation_x(std::f32::consts::PI / 2.0), // Top
//             3 => Quat::from_rotation_x(-std::f32::consts::PI / 2.0), // Bottom
//             4 => Quat::from_rotation_y(-std::f32::consts::PI / 2.0), // Left
//             _ => Quat::from_rotation_y(std::f32::consts::PI / 2.0),  // Right
//         };

//         commands.spawn((
//             PbrBundle {
//                 mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0, subdivisions: todo!() })),
//                 material: materials.add(StandardMaterial {
//                     base_color: color,
//                     ..default()
//                 }),
//                 transform: Transform {
//                     rotation,
//                     translation: match i {
//                         0 => Vec3::new(0.0, 0.0, 0.5), // Front
//                         1 => Vec3::new(0.0, 0.0, -0.5), // Back
//                         2 => Vec3::new(0.0, 0.5, 0.0), // Top
//                         3 => Vec3::new(0.0, -0.5, 0.0), // Bottom
//                         4 => Vec3::new(-0.5, 0.0, 0.0), // Left
//                         _ => Vec3::new(0.5, 0.0, 0.0),  // Right
//                     },
//                     ..default()
//                 },
//                 ..default()
//             },
//             Cube,
//         )); 
//     }
// }

// pub fn rotate_cube(mut query: Query<&mut Transform, With<Cube>>, keys: Res<Input<KeyCode>>, time: Res<Time>) {
//     let mut rotation = Vec3::ZERO;

//     if keys.pressed(KeyCode::Left) {
//         rotation.y += 1.0;
//     }
//     if keys.pressed(KeyCode::Right) {
//         rotation.y -= 1.0;
//     }
//     if keys.pressed(KeyCode::Up) {
//         rotation.x += 1.0;
//     }
//     if keys.pressed(KeyCode::Down) {
//         rotation.x -= 1.0;
//     }

//     if rotation.length() > 0.0 {
//         for mut transform in query.iter_mut() {
//             transform.rotation *= Quat::from_rotation_y(rotation.y * time.delta_seconds())
//                 * Quat::from_rotation_x(rotation.x * time.delta_seconds());
//         }
//     }
// }
