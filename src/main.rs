// use rubiks::rubiks::cube::RubiksCube;
use crate::rubiks::cube::RubiksCube;
// use std::time::Instant;
// use cfop::cfop::cross::solve_cross; // Import the function
use crate::cfop::cross::solve_cross; // Import the function

fn main() {
    // Create new instance of the Cube
    let mut cube = RubiksCube::new();
    // Define the scramble in the standard notation
    let scramble = "L R U D";
    // Scramble the Cube
    cube.apply_scramble(scramble);

    println!("{:?}", cube);

    // // Solve the cube using BFS
    // let start_time = Instant::now();
    // if let Some(solution) = cube.solve() {
    //     println!("Solution found in {} moves: {:?}", solution.len(), solution);
    // } else {
    //     println!("No solution found.");
    // }
    // let elapsed_time = start_time.elapsed();
    // println!("Elapsed time: {:?}", elapsed_time);

    // now we solve the cube using CFOP
    println!("{:?}", solve_cross(cube));

    // Visualize scrambled cube
    cube.clone().visualize();
}
