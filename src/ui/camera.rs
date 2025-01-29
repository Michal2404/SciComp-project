// This file contains information on the camera setting
use crate::ui::app::CubeSettings;

use bevy::input::mouse::{MouseScrollUnit, MouseWheel, MouseMotion};
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use std::f32::consts::TAU;


pub fn spawn_camera(
    mut commands: Commands,
    cube_settings: Res<CubeSettings>,
) {
    /*
    This function sets up the initial position of the camera
     */
    //camera
    commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(cube_settings.camera_x, cube_settings.camera_y, cube_settings.camera_z).looking_at(Vec3::ZERO, Vec3::Y),

    ));
}

pub fn zoom_camera(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
    cube_settings: Res<CubeSettings>,
) {
    for ev in scroll_evr.read() {
        let mut transform = q_camera.single_mut();
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.x + ev.y > 0.0 {
                    transform.translation.x =
                        cube_settings.camera_zoom_speed * transform.translation.x;
                    transform.translation.y =
                        cube_settings.camera_zoom_speed * transform.translation.y;
                    transform.translation.z =
                        cube_settings.camera_zoom_speed * transform.translation.z;
                } else {
                    transform.translation.x =
                        transform.translation.x / cube_settings.camera_zoom_speed;
                    transform.translation.y =
                        transform.translation.y / cube_settings.camera_zoom_speed;
                    transform.translation.z =
                        transform.translation.z / cube_settings.camera_zoom_speed;
                }
            }
            MouseScrollUnit::Pixel => {
                if ev.x + ev.y > 0.0 {
                    transform.translation.x =
                        cube_settings.camera_zoom_speed * transform.translation.x;
                    transform.translation.y =
                        cube_settings.camera_zoom_speed * transform.translation.y;
                    transform.translation.z =
                        cube_settings.camera_zoom_speed * transform.translation.z;
                } else {
                    transform.translation.x =
                        transform.translation.x / cube_settings.camera_zoom_speed;
                    transform.translation.y =
                        transform.translation.y / cube_settings.camera_zoom_speed;
                    transform.translation.z =
                        transform.translation.z / cube_settings.camera_zoom_speed;
                }
            }
        }
    }
}

//------------------------------------------------Mouse Rotation-------------------------------------------------------
#[derive(Debug, Resource)]
pub struct MouseDraggingRecorder {
    pub start_pos: Option<Vec3>,
    pub piece: Option<Entity>,
    // pub triggered: bool,
}

// impl MouseDraggingRecorder {
//     pub fn clear(&mut self) {
//         self.start_pos = None;
//         self.piece = None;
//         self.triggered = false;
//     }
// }
pub fn move_camera(
    mut q_camera: Query<&mut Transform, With<Camera>>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    recorder: Res<MouseDraggingRecorder>,
    mut egui_context: EguiContexts,
) {
    // skip the movement of camera if use is interacting with the egui
    let ctx = egui_context.ctx_mut(); // Access egui's context
    if ctx.wants_pointer_input() {
        // Skip rotation if egui is interacting with the pointer
        return;
    }

    if buttons.pressed(MouseButton::Left) {
        if recorder.piece.is_none() || recorder.start_pos.is_none() {
            // println!("move camera");
            for motion in motion_evr.read() {
                // motion.delta.x Sliding the mouse left is negative, sliding right is positive
                // motion.delta.y When the mouse slides up, it is negative, when it slides down, it is positive.
                for mut transform in &mut q_camera {
                    // println!("camera translation: {}, motion.delta: {}", transform.translation, motion.delta);
                    if motion.delta.x.abs() > 0.001 {
                        // For horizontal rotation, the camera only needs to rotate around the y-axis
                        let max = transform
                            .translation
                            .x
                            .abs()
                            .max(transform.translation.y.abs())
                            .max(transform.translation.z.abs());
                        let quat = Quat::from_euler(
                            EulerRot::XYZ,
                            0.0,
                            0.0002 * -motion.delta.x * max * TAU, // Multiplying by max is to maintain the same rate as sliding up and down
                            0.0,
                        );
                        transform.rotate_around(Vec3::ZERO, quat);
                    }
                    if motion.delta.y.abs() > 0.001 {
                        // Vertical rotation requires rotation around the x-axis and z-axis at the same time, and the rotation angle is inversely proportional to the angle with the coordinate axis.
                        let quat = Quat::from_euler(
                            EulerRot::XYZ,
                            0.0002 * -motion.delta.y * transform.translation.z * TAU,
                            0.0,
                            0.0002 * motion.delta.y * transform.translation.x * TAU,
                        );
                        transform.rotate_around(Vec3::ZERO, quat);
                    }
                }
            }
        }
    }
    motion_evr.clear();
}

