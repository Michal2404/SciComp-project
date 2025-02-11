// This file configures the cubie pieces and displays it
use crate::ui::app::CubeSettings;

use bevy::prelude::*;
use std::f32::consts::{PI, FRAC_PI_2};



//------------------------------------------------Cube Stuff-------------------------------------------------------
#[derive(Debug, Component, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Cubie {
    pub id: usize,
    pub original_position: Vec3,
    pub current_position: Vec3,
    pub left_angle: f32,
}

impl Cubie {
    pub fn spawn(
        &self, 
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        cube_settings: &Res<CubeSettings>,
        // child: &bool,
    ){
        /*
        This function spawns in the cubie
         */

        let r = 0.5;
        let g = 0.5;
        let b = 0.5;
        // let r = 1.0;
        // let g = 1.0;
        // let b = 1.0;
        let a = 0.1;
        // create cubie
        // if *child{
            commands
                .spawn((
                    Mesh3d(meshes.add(Cuboid::new(cube_settings.piece_size, cube_settings.piece_size, cube_settings.piece_size).mesh())),
                    // MeshMaterial3d(materials.add(Color::BLACK)),
                    MeshMaterial3d(materials.add(Color::srgba(r, g, b, a))),
                    Transform::from_translation(Vec3::new(self.original_position[0], self.original_position[1], self.original_position[2])),
                    Cubie{
                        id: self.id,
                        original_position: self.original_position,
                        current_position: self.current_position,
                        left_angle: self.left_angle
                    },
                    RayCastPickable,
                ))
                .with_children(|parent| {
                    self.spawn_stickers(parent, meshes, materials, cube_settings);
                });
            // }
            // else {
            // commands
            //     .spawn((
            //         Mesh3d(meshes.add(Cuboid::new(cube_settings.piece_size, cube_settings.piece_size, cube_settings.piece_size).mesh())),
            //         // MeshMaterial3d(materials.add(Color::BLACK)),
            //         MeshMaterial3d(materials.add(Color::srgba(r, g, b, a))),
            //         Transform::from_translation(Vec3::new(self.original_position[0], self.original_position[1], self.original_position[2])),
            //         Cubie{
            //             id: self.id,
            //             original_position: self.original_position,
            //             current_position: self.current_position,
            //             left_angle: self.left_angle
            //         },
            //         RayCastPickable,
            //     ));
        // }
        
        
    }

    pub fn spawn_stickers(
        &self,
        parent: &mut ChildBuilder,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        cube_settings: &CubeSettings,
    ) {    
        if self.has_up_face() {
        // if self.has_up_face(cube_settings) {
            let mut transform = Transform::from_translation(Vec3::new(
                0.0, 
                0.5 * cube_settings.piece_size + 0.01, 
                0.0,
            ));
            transform.rotate_x(-PI);
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(cube_settings.sticker_size, 0.01, cube_settings.sticker_size).mesh())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: cube_settings.up_color,
                    unlit: true,
                    ..default()
                })),
                transform,
            ));
        }
    
        if self.has_down_face() {
        // if self.has_down_face(cube_settings) {
            let mut transform = Transform::from_translation(Vec3::new(
                0.0,
                -0.5 * cube_settings.piece_size - 0.01,
                0.0,
            ));
            transform.rotate_x(PI);
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(cube_settings.sticker_size, 0.01, cube_settings.sticker_size).mesh())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: cube_settings.down_color,
                    unlit: true,
                    ..default()
                })),
                transform,
            ));
        }
        if self.has_left_face() {
        // if self.has_left_face(cube_settings) {
            let mut transform = Transform::from_translation(Vec3::new(
                -0.5 * cube_settings.piece_size - 0.01,
                0.0,
                0.0,
            ));
            transform.rotate_z(FRAC_PI_2);
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(cube_settings.sticker_size, 0.01, cube_settings.sticker_size).mesh())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: cube_settings.left_color,
                    unlit: true,
                    ..default()
                })),
                transform,
            ));
        }
    
        if self.has_right_face() {
        // if self.has_right_face(cube_settings) {
            let mut transform = Transform::from_translation(Vec3::new(
                0.5 * cube_settings.piece_size + 0.01, 
                0.0, 
                0.0,
            ));
            transform.rotate_z(-FRAC_PI_2);
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(cube_settings.sticker_size, 0.01, cube_settings.sticker_size).mesh())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: cube_settings.right_color,
                    unlit: true,
                    ..default()
                })),
                transform,
            ));
        }
    
        if self.has_front_face() {
        // if self.has_front_face(cube_settings) {
            let mut transform = Transform::from_translation(Vec3::new(
                0.0, 
                0.0, 
                0.5 * cube_settings.piece_size + 0.01,
            ));
            transform.rotate_x(FRAC_PI_2);
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(cube_settings.sticker_size, 0.01, cube_settings.sticker_size).mesh())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: cube_settings.front_color,
                    unlit: true,
                    ..default()
                })),
                transform,
            ));
        }
    
        if self.has_back_face() {
        // if self.has_back_face(cube_settings) {
            let mut transform = Transform::from_translation(Vec3::new(
                0.0,
                0.0,
                -0.5 * cube_settings.piece_size - 0.01,
            ));
            transform.rotate_x(-FRAC_PI_2);
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(cube_settings.sticker_size, 0.01, cube_settings.sticker_size).mesh())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: cube_settings.back_color,
                    unlit: true,
                    ..default()
                })),
                transform,
            ));
        }
    }

    pub fn has_up_face(&self) -> bool {
    // pub fn has_up_face(&self, cube_settings: &CubeSettings) -> bool {
        // self.original_position[1] == 1.0 + cube_settings.cube_y
        self.original_position[1] == 1.0
    }
    pub fn has_down_face(&self) -> bool {
        // pub fn has_down_face(&self, cube_settings: &CubeSettings) -> bool {
        // self.original_position[1] == -1.0 + cube_settings.cube_y
        self.original_position[1] == -1.0
    }
    pub fn has_left_face(&self) -> bool {
        // pub fn has_left_face(&self, cube_settings: &CubeSettings) -> bool {
        // if self.original_position[0] == -1.0 && self.original_position[1] == 0.0 && self.original_position[2] == 0.0 {
        //     return true;
        // }
        // self.original_position[0] == -1.0 + cube_settings.cube_x
        self.original_position[0] == -1.0
        // false
    }
    pub fn has_right_face(&self) -> bool {
    // pub fn has_right_face(&self, cube_settings: &CubeSettings) -> bool {
        // if self.original_position[0] == 1.0 && self.original_position[1] == 0.0 && self.original_position[2] == 0.0 {
        //     return true;
        // }
        // self.original_position[0] == 1.0 + cube_settings.cube_x
        self.original_position[0] == 1.0
        // false
    }
    pub fn has_front_face(&self) -> bool {
    // pub fn has_front_face(&self, cube_settings: &CubeSettings) -> bool {
        // if self.original_position[2] == 1.0 && self.original_position[0] == 0.0 && self.original_position[1] == 0.0 {
        //     return true;
        // }
        // self.original_position[2] == 1.0 + cube_settings.cube_z
        self.original_position[2] == 1.0
        // false
    }
    pub fn has_back_face(&self) -> bool {
    // pub fn has_back_face(&self, cube_settings: &CubeSettings) -> bool {
        // if self.original_position[2] == -1.0 && self.original_position[0] == 0.0 && self.original_position[1] == 0.0 {
        //     return true;
        // }
        // self.original_position[2] == -1.0 + cube_settings.cube_z
        self.original_position[2] == -1.0
        // false
    }
}

pub fn spawn_rubiks_cube(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_settings: Res<CubeSettings>,
) {
    /*
    This function spawns in the rubiks
    */
    // initialize cubie id
    let mut id = 0;
    // let mut child = true;
    // we loop through each cubie
    for x in [-1.0, 0.0, 1.0] {
        for y in [-1.0, 0.0, 1.0] {
            for z in [-1.0, 0.0, 1.0] {
    // for x in [-1.0+cube_settings.cube_x, 0.0+cube_settings.cube_x, 1.0+cube_settings.cube_x] {
        // for y in [-1.0+cube_settings.cube_y, 0.0+cube_settings.cube_y, 1.0+cube_settings.cube_y] {
            // for z in [-1.0+cube_settings.cube_z, 0.0+cube_settings.cube_z, 1.0+cube_settings.cube_z] {
                // if x == 0.0 && y == 0.0 && z == 0.0 {
                //     continue;
                // }
                
                // for visual
                // scramble
                // if (y == 0.0 && ((z == 0.0 && x != 0.0) || (x == 0.0 && z != 0.0))) ||
                // (y != 0.0 && z == 0.0 && x == 0.0)

                // cross
                // if (y == 0.0 && ((z == 0.0 && x != 0.0) || (x == 0.0 && z != 0.0))) ||
                // (y == -1.0 && ((z == 0.0 && x != 0.0) || (x == 0.0 && z != 0.0))) ||
                // (y == -1.0 && x == 0.0 && z == 0.0) ||
                // (y == 1.0 && x == 0.0 && z == 0.0)
                
                // f2l
                // if y != 1.0 ||
                // (y == 1.0 && x == 0.0 && z == 0.0)
                // {
                //     child = true;
                // } else {
                //     child = false;
                // }

                // initialize cubie
                let cubie = Cubie {
                    id,
                    original_position: Vec3::new(x, y, z),
                    current_position: Vec3::new(x, y, z),
                    left_angle: 0.0
                };
                // create cubie
                // cubie.spawn(&mut commands, &mut meshes, &mut materials, &cube_settings, &child);
                cubie.spawn(&mut commands, &mut meshes, &mut materials, &cube_settings);

                // update id
                id += 1
            }
        }
    }
}