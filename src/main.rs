mod rubiks;
mod cfop;
mod ui;
mod a_star;
mod helper;

use std::time::Instant;

use rubiks::cube::RubiksCube;
use helper::utils::*;
use ui::app::*;
use bevy::prelude::*;
use cfop::total::cfop_solver;
use a_star::a_star::a_star_solver;
use a_star::bidirectional_a_star::bidirectional_a_star_solver;
use a_star::parallel_a_star::parallel_a_star_solver;
use a_star::iterative_deep_a_star::ida_star_solver;



fn main() {
    // Create new instance of the Cube
    let mut cube = RubiksCube::new();
    // let mut cube1 = RubiksCube::new();
    // let mut cube2 = RubiksCube::new();
    
    // Define the scramble in the standard notation
    // let scramble = "L R U D";
    // let scramble = "L R U D U";
    let scramble = "L2 U R U2";

    // let scramble = "L R U D B L R";

    // let scramble = "R2 F R B D R F2 L F' D2 L U2 R B2 R2 F2 R2 U2 R' D2 L2";
    // let scramble = "R U B2 L D' R' U' B R U F2 B2 U' R2 D' R2 F2 U' L2 B2 U2";
    // let scramble = "B U2 L' B L2 U' B2 U' F L U2 R2 D F2 U' L2 D R2 F2 B2 U2";
    // let scramble = "B2 R U' B D F2 R2 D' L F R2 F2 D2 L2 U2 R2 B D2 F B2 R2";
    // let scramble = "D' L2 U L D' B' D F2 L' R2 D B2 U2 F2 R2 U' R2 D2 F2 U' F2";
    // let scramble = "R U L2 F D2 F L U' L2 F' B' L2 B U2 B' U2 R2 U2 F2 R D";
    // let scramble = "B' L' R2 U B2 R2 U' R2 F2 D2 R2 L' F R2 F' D B D";
    // let scramble = "";
    
    // Scramble the Cube
    cube.apply_scramble(scramble);
    // cube1.apply_scramble(scramble);
    // cube2.apply_scramble(scramble);
    
    // Solve the cube using CFOP
    cfop_solver(scramble, cube);
    // println!("-------------parallel a-star-------------");
    // let start_time = Instant::now();
    // let solved = ida_star_solver(scramble, &mut cube2);
    // let elapsed_time = start_time.elapsed();
    // println!("{}", solved.join(" "));
    // println!("Elapsed time: {:?}", elapsed_time);

    // println!("-------------a-star-------------");
    // let start_time = Instant::now();
    // let solved = a_star_solver(scramble, &mut cube1);
    // let elapsed_time = start_time.elapsed();
    // println!("{}", solved.join(" "));
    // println!("Elapsed time: {:?}", elapsed_time);
    
    
    // Visualize scrambled cube
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, setup)
        .run()

}
