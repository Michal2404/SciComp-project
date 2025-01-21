use crate::ui::pieces::*;
use crate::ui::camera::*;
use crate::ui::rotate::*;
use crate::ui::design::*;

use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::color::palettes;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Resource)]
pub struct CubeSettings {
    pub front_color: Color,
    pub back_color: Color,
    pub left_color: Color,
    pub right_color: Color,
    pub up_color: Color,
    pub down_color: Color,
    // pub piece_size: f64,
    // pub sticker_size: f64,
    // pub camera_zoom_speed: f64,
    // pub rotate_speed: f64,
    pub piece_size: f32,
    pub sticker_size: f32,
    pub camera_zoom_speed: f32,
    pub rotate_speed: f32,
}

impl Default for CubeSettings {
    fn default() -> Self {
        Self {
            front_color: palettes::css::GREEN.into(),
            back_color: palettes::css::BLUE.into(),
            left_color: palettes::css::ORANGE.into(),
            right_color: palettes::css::RED.into(),
            up_color: palettes::css::WHITE.into(),
            down_color: palettes::css::YELLOW.into(),
            piece_size: 1.0,
            sticker_size: 0.9,
            // play_mode: PlayMode::Practice,
            camera_zoom_speed: 1.1,
            rotate_speed: 0.5,
        }
    }
}

pub fn run_visualization(run: bool) {
    /*
    This function runs the visualization for rubiks cube
     */
    // let moves: Vec<String> = vec!["U B F L' U2 B"].iter().flat_map(|s| s.split_whitespace()).map(|s| s.to_string()).collect();
    let moves: Vec<String> = vec!["U"].iter().flat_map(|s| s.split_whitespace()).map(|s| s.to_string()).collect();

    if run {
    // Visualize scrambled cube
    App::new()
    .add_plugins((DefaultPlugins, MeshPickingPlugin))
    .add_plugins(EguiPlugin)
    // .add_plugins(WorldInspectorPlugin::new())
    .insert_resource(MeshPickingSettings {
        require_markers: true,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
    .insert_resource(CubeSettings::default())
    .insert_resource(InputText::default())
    .insert_resource(MoveQueue(VecDeque::new()))
    .insert_resource(Rotation::default())
    .insert_resource(MouseDraggingRecorder {
        start_pos: None,
        piece: None,
        triggered: false,
    })
    .register_type::<Cubie>()
    .add_event::<ScrambleEvent>()
    .add_event::<GenerateScrambleEvent>()
    .add_systems(Startup, 
        (
            spawn_camera, 
            spawn_rubiks_cube
        ))
    .add_systems(PreUpdate,
    (
        plan_move,
        ).run_if(check_field)
    )
    .add_systems(Update, 
        (
            rotate_cube,
            zoom_camera,
            move_camera,
            game_ui,
            scramble_cube,
            // generate_random_scramble,
        ))
    .add_systems(PostUpdate,
        (
            piece_translation_round,
        )
        .run_if(check_field)
        // .after(TransformSystem::TransformPropagate,),
    )
    // .add_systems(
    //     PostUpdate,
    //     ((
    //         piece_translation_round,
    //         cleanup_movable_pieces.after(piece_translation_round),
    //     )
    //         .after(TransformSystem::TransformPropagate),),
    // )
    .run();
    }
}

fn check_field(resource: Res<Rotation>) -> bool {
    if resource.completed == true {
        return true
    }
    else {
        return false
    }
}