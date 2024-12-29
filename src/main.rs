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

fn main() {
    // Create new instance of the Cube
    let mut cube = RubiksCube::new();
    // Define the scramble in the standard notation
    // let scramble = "U";
    // let scramble = "L R U D";
    let scramble = "U L B' D D B B U B' L L U R R U' R R D D L L U U R R F F D' R R B' L'";
    // Scramble the Cube
    cube.apply_scramble(scramble);

    // cube.clone().visualize();

    // Solve the cube using CFOP
    // Determine the color of the bottom face
    let target = cube.faces[1][4];

    // Step 1: Solve the cross
    println!("-------------cross-------------");
    let start_time = Instant::now();
    let cube_cross = solve_cross(&mut cube, &target);
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);
    
    // Step 2: Solve the first 2 layers
    println!("-------------f2l-------------");
    solve_f2l(&cube_cross, &target);
    
    // Visualize scrambled cube
    cube_cross.clone().visualize();
}
