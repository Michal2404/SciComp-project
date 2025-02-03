use crate::ui::pieces::*;
use crate::ui::camera::*;
use crate::ui::rotate::*;
use crate::ui::design::*;

use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::color::palettes;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Resource)]
pub struct CubeSettings {
    pub front_color: Color, // color of front side on rubiks cube
    pub back_color: Color, // color of back side on rubiks cube 
    pub left_color: Color, // color of left side on rubiks cube
    pub right_color: Color, // color of right side on rubiks cube
    pub up_color: Color, // color of up side on rubiks cube
    pub down_color: Color, // color of down side on rubiks cube
    pub piece_size: f32, // piece size of rubiks cube
    pub sticker_size: f32, // sticker size of rubiks cube
    pub camera_zoom_speed: f32, // speed to zoom in/out
    pub rotate_speed: f32, // speed of rubiks cube rotation
    pub num_scramble_moves: usize, // number of scramble moves
    pub camera_x: f32, // camera position x
    pub camera_y: f32, // camera position y
    pub camera_z: f32, // camera position z
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
            camera_zoom_speed: 1.05,
            rotate_speed: 1.0,
            num_scramble_moves: 5,
            camera_x: 6.0,
            camera_y: 6.0,
            camera_z: 6.0,
        }
    }
}

pub fn run_visualization(run: bool) {
    /*
    This function runs the visualization for rubiks cube
     */

    if run {
    // Visualize scrambled cube
    App::new()
    .add_plugins((DefaultPlugins, MeshPickingPlugin, EguiPlugin))
    // .add_plugins(WorldInspectorPlugin::new())
    .insert_resource(MeshPickingSettings {
        require_markers: true,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
    .insert_resource(CubeSettings::default())
    .insert_resource(Scramble::default())
    .insert_resource(SolveData::default())
    .insert_resource(SolveTask(None))
    .insert_resource(MoveQueue(VecDeque::new()))
    .insert_resource(Rotation::default())
    .insert_resource(TimekeepingTimer::default())
    .insert_resource(MouseDraggingRecorder {
        start_pos: None,
        piece: None,
        // triggered: false,
    })
    .register_type::<Cubie>()
    .add_event::<ScrambleEvent>()
    .add_event::<ResetEvent>()
    .add_event::<CFOPEvent>()
    .add_event::<ASTAREvent>()
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
            zoom_camera,
            move_camera,
            game_ui,
            rotate_cube,
            scramble_cube,
            reset_cube,
            solve_cfop,
            solve_astar,
            poll_solve_task,
        ))
    .add_systems(PostUpdate,
        (
            piece_translation_round.after(TransformSystem::TransformPropagate,)
        )
        // .after(TransformSystem::TransformPropagate,)
        .run_if(check_field)
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