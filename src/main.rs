// use rubiks::rubiks::cube::RubiksCube;
// // use std::time::Instant;
// // use cfop::cfop::cross::solve_cross; // Import the function
// use cfop::cfop::cross::solve_cross; // Import the function

use std::time::Instant;
mod rubiks;
mod cfop;
mod ui;

use rubiks::cube::RubiksCube;
use cfop::cross::solve_cross;
use cfop::f2l::solve_f2l;
use cfop::oll::solve_oll;

fn main() {
    // Create new instance of the Cube
    let mut cube = RubiksCube::new();
    // Define the scramble in the standard notation
    // let scramble = "U";
    let scramble = "L R U D";
    // let scramble = "U L B' D D B B U B' L L U R R U' R R D D L L U U R R F F D' R R B' L'";
    // Scramble the Cube
    cube.apply_scramble(scramble);

    // cube.clone().visualize();

    // Solve the cube using CFOP
    // Determine the color of the bottom and top face
    let bottom = cube.faces[1][4];
    let top = cube.faces[0][4];

    // Step 1: Solve the cross
    println!("-------------cross-------------");
    let start_time = Instant::now();
    // let mut cube_cross = solve_cross(&mut cube, &target);
    solve_cross(&mut cube, &bottom);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    
    // Step 2: Solve the first 2 layers
    println!("-------------F2L-------------");
    let start_time = Instant::now();
    // let mut cube_f2l = solve_f2l(&mut cube_cross, &target);
    solve_f2l(&mut cube, &bottom);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    
    // Step 3: Solve OLL
    println!("-------------OLL-------------");
    let start_time = Instant::now();
    solve_oll(&mut cube, &top);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);


    // Visualize scrambled cube
    cube.clone().visualize();
}
