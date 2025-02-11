use std::collections::VecDeque;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

// // This file performs rotation to the cube depending on the move
use bevy::prelude::*;
// use crate::rubiks::cube;
use crate::ui::pieces::Cubie;
use crate::ui::app::CubeSettings;
use crate::ui::design::SolveData;
use crate::ui::design::SolverInformation;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Clockwise90,
    Clockwise180,
    Counterclockwise90,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Component, Clone, Copy, Resource)]
pub struct Rotation {
    pub axis: Axis,
    pub direction: Direction, 
    pub position: f32,
    pub completed: bool 
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            axis: Axis::X,
            direction: Direction::Clockwise90,
            position: 3.0,
            completed: true
        }
    }
}


#[derive(Debug, Resource)]
pub struct MoveQueue(pub VecDeque<String>);


impl Rotation {
    pub fn rotate(
        &mut self,
        time: &Res<Time>, 
        query: &mut Query<(&mut Transform, &mut Cubie), With<Cubie>>,
        cube_settings: &Res<CubeSettings>,
    ){
        /*
        This function rotates the cube depending on the axis, direction, and position on which to rotate
         */
        // get the necessary constants
        let (location, angle_constant, axis_vec) = self.data(cube_settings);
        for (mut transform, mut cubie) in query.iter_mut() {
            // rotate pieces that have specific position
                if cubie.current_position[location.unwrap()] == self.position {
                    let mut angle = angle_constant.unwrap() * cube_settings.rotate_speed * TAU * time.delta_secs();
                    let mut new_left_angle = cubie.left_angle + angle;
                    // if we exceeded movement, we stop
                    // for condition 1
                    if ((self.direction == Direction::Clockwise90 || self.direction == Direction::Clockwise180) && self.position == -1.0 && new_left_angle >= 0.0) ||
                    (self.direction == Direction::Counterclockwise90 && self.position == 1.0 && new_left_angle >= 0.0)
                    // if ((self.direction == Direction::Clockwise90 || self.direction == Direction::Clockwise180) && self.position == -1.0 && new_left_angle >= max_movement.unwrap()) ||
                    // (self.direction == Direction::Counterclockwise90 && self.position == 1.0 && new_left_angle >= max_movement.unwrap())
                    {
                        angle = cubie.left_angle;
                        new_left_angle = 0.0;
                        self.completed = true;
                    }
                    // for condition 2
                    else if (self.direction == Direction::Counterclockwise90 && self.position == -1.0 && new_left_angle <= 0.0) ||
                    ((self.direction == Direction::Clockwise90 || self.direction == Direction::Clockwise180) && self.position == 1.0 && new_left_angle <= 0.0)
                    // if (self.direction == Direction::Counterclockwise90 && self.position == -1.0 && new_left_angle <= -max_movement.unwrap()) ||
                    // ((self.direction == Direction::Clockwise90 || self.direction == Direction::Clockwise180) && self.position == 1.0 && new_left_angle <= -max_movement.unwrap())
                    {
                        angle = cubie.left_angle;
                        new_left_angle = 0.0;
                        self.completed = true;
                    }
                    transform.rotate_around(Vec3::new(0.0, 0.0, 0.0), Quat::from_axis_angle(axis_vec.unwrap(), angle));
                    // transform.rotate_around(Vec3::new(cube_settings.cube_x, cube_settings.cube_y, cube_settings.cube_z), Quat::from_axis_angle(axis_vec.unwrap(), angle));
                    cubie.left_angle = new_left_angle;
                    
            }
        }       
        
    }
    pub fn data(&self, cube_settings: &CubeSettings) -> (Option<usize>, Option<f32>, Option<Vec3>){
        /*
        This function outputs the necessary data needed for each part of the rotation
        */
        // get the location (0, 1, 2)
        let location = match self.axis {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        };
        // get direction
        let mut angle_constant = match self.direction {
            Direction::Clockwise90 | Direction::Clockwise180 => 1.0,
            Direction::Counterclockwise90 => -1.0
        };
        // if the position value is 1.0, we multiply angle_constant by -1
        if self.position == 1.0 {angle_constant *= -1.0};
        // get axis
        let axis_vec = match self.axis {
            // Axis::X => Vec3::new(1.0, cube_settings.cube_y, cube_settings.cube_z),
            // Axis::Y => Vec3::new(cube_settings.cube_x, 1.0, cube_settings.cube_z),
            // Axis::Z => Vec3::new(cube_settings.cube_x, cube_settings.cube_y, 1.0),
            Axis::X => Vec3::X,
            Axis::Y => Vec3::Y,
            Axis::Z => Vec3::Z,
        };
        
        // return these
        (Some(location), Some(angle_constant), Some(axis_vec))
        
    }

    fn angle(&self) -> f32 {
        /*
        This function finds the angle we want to initially use
         */
        if (self.direction == Direction::Clockwise90 && self.position == -1.0) ||
        (self.direction == Direction::Counterclockwise90 && self.position == 1.0)
        {
            return -FRAC_PI_2
        }
        // for condition 2
        else if (self.direction == Direction::Counterclockwise90 && self.position == -1.0) ||
        (self.direction == Direction::Clockwise90 && self.position == 1.0)
        {
            return FRAC_PI_2
        }
        else if self.direction == Direction::Clockwise180 && self.position == -1.0 {
            return -PI
        }
        else if self.direction == Direction::Clockwise180 && self.position == 1.0 {
            return PI
        }
        else {
            return 0.0
        }

    }
}

pub fn plan_move(
    mut query: Query<(&mut Transform, &mut Cubie), With<Cubie>>,
    mut side_move_queue: ResMut<MoveQueue>,
    mut rotation: ResMut<Rotation>,
) {
    /*
    This function determines what move to perform next
     */
    // first we see if the query isn't empty
    if !side_move_queue.0.is_empty(){
        // pop the first move
        // let temp = side_move_queue.0[0].clone();
        let temp = side_move_queue.0.pop_front().unwrap();
        let notation = temp.as_str();
        // convert into rotation to see which cubie we need to update the agle
        let rotation_temp = moves(notation);
        // update rotation
        rotation.axis = rotation_temp.axis;
        rotation.direction = rotation_temp.direction;
        rotation.position = rotation_temp.position;
        // set rotation completed to false
        rotation.completed = false;
        for (_, mut cubie) in query.iter_mut() {
            cubie.left_angle = rotation.angle();
        }
    }
}

pub fn piece_translation_round(
    mut cube: Query<(&mut Transform, &mut Cubie), Without<Parent>>,
    // mut solve_data: ResMut<SolveData>,
    // mut cube: Query<(&mut Transform, &mut Cubie)>,
) {
    /*
    This function cleans up moves after transformation
     */
    for (mut transform, mut cubie) in &mut cube {
        if cubie.left_angle == 0.0 {
            // fix up translation
            cubie.current_position = Vec3::new(transform.translation.x.round(), transform.translation.y.round(), transform.translation.z.round());
            // cubie.current_position = Vec3::new(transform.translation.x, transform.translation.y, transform.translation.z);
            transform.translation.x = transform.translation.x.round();
            transform.translation.y = transform.translation.y.round();
            transform.translation.z = transform.translation.z.round(); 

            // Extract the current rotation as Euler angles
            let (mut pitch, mut yaw, mut roll) = transform.rotation.to_euler(EulerRot::YXZ);

            // Snap each angle to the nearest multiple of 90° (π/2 radians)
            pitch = (pitch / FRAC_PI_2).round() * FRAC_PI_2;
            yaw = (yaw / FRAC_PI_2).round() * FRAC_PI_2;
            roll = (roll / FRAC_PI_2).round() * FRAC_PI_2;

            // Reconstruct the corrected quaternion
            transform.rotation = Quat::from_euler(EulerRot::YXZ, pitch, yaw, roll); 
            
        }
    }
    
}

pub fn rotate_cube(
    time: Res<Time>, 
    mut query: Query<(&mut Transform, &mut Cubie), With<Cubie>>,
    cube_settings: Res<CubeSettings>,
    mut rotation: ResMut<Rotation>,
){
    /*
    This function rotates the cube depending on which moves we perform
    */
    if !rotation.completed {
        rotation.rotate(&time, &mut query, &cube_settings);
    } 
}

fn moves(notation: &str) -> Rotation {
    /*
    This function converts notation into Rotation struct
     */
    let first_letter = &notation[0..1];

    let mut rotation = match first_letter {
        "U" => Rotation {
            axis: Axis::Y,
            direction: Direction::Clockwise90,
            position: 1.0,
            completed: false,
        },
        "D" => Rotation {
            axis: Axis::Y,
            direction: Direction::Clockwise90,
            position: -1.0,
            completed: false,
        },
        "F" => Rotation {
            axis: Axis::Z,
            direction: Direction::Clockwise90,
            position: 1.0,
            completed: false,
        },
        "B" => Rotation {
            axis: Axis::Z,
            direction: Direction::Clockwise90,
            position: -1.0,
            completed: false,
        },
        "R" => Rotation {
            axis: Axis::X,
            direction: Direction::Clockwise90,
            position: 1.0,
            completed: false,
        },
        "L" => Rotation {
            axis: Axis::X,
            direction: Direction::Clockwise90,
            position: -1.0,
            completed: false,
        },
        _ => Rotation {
            axis: Axis::X,
            direction: Direction::Clockwise90,
            position: 3.0,
            completed: false,
        }
    };

    // if position == 3.0, then something went wrong
    assert!(rotation.position != 3.0, "something went wrong here");

    // now we check if notation contains a prime or 2
    if notation.ends_with("'"){
        rotation.direction = Direction::Counterclockwise90;
    }
    if notation.ends_with("2"){
        rotation.direction = Direction::Clockwise180;
    }

    rotation
}