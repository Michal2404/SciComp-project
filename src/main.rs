use rubiks::rubiks::cube::RubiksCube;
use std::time::Instant;

fn main() {
    // Create new instance of the Cube
    let mut cube = RubiksCube::new();
    // Define the scramble in the standard notation
    let scramble = "L R U D";
    // Scramble the Cube
    cube.apply_scramble(scramble);

    // Solve the cube using BFS
    let start_time = Instant::now();
    if let Some(solution) = cube.solve() {
        println!("Solution found in {} moves: {:?}", solution.len(), solution);
    } else {
        println!("No solution found.");
    }
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    // Visualize scrambled cube
    cube.clone().visualize();
}
