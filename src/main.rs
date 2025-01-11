use std::time::Instant;
mod rubiks;
mod cfop;
mod ui;

use rubiks::cube::RubiksCube;
use cfop::cross::solve_cross;
use cfop::f2l::solve_f2l;
use cfop::oll::solve_oll;
use cfop::pll::solve_pll;
use ui::app::*;
use bevy::prelude::*;
use cfop::helper::{cleanup_moves, output_data};

fn main() {
    // Create new instance of the Cube
    let mut cube = RubiksCube::new();

    // Define the scramble in the standard notation
    // let scramble = "L R U D";
    // let scramble = "R2 F R B D R F2 L F' D2 L U2 R B2 R2 F2 R2 U2 R' D2 L2";
    // let scramble = "R U B2 L D' R' U' B R U F2 B2 U' R2 D' R2 F2 U' L2 B2 U2";
    // let scramble = "B U2 L' B L2 U' B2 U' F L U2 R2 D F2 U' L2 D R2 F2 B2 U2";
    // let scramble = "B2 R U' B D F2 R2 D' L F R2 F2 D2 L2 U2 R2 B D2 F B2 R2";
    // let scramble = "D' L2 U L D' B' D F2 L' R2 D B2 U2 F2 R2 U' R2 D2 F2 U' F2";
    // let scramble = "R U L2 F D2 F L U' L2 F' B' L2 B U2 B' U2 R2 U2 F2 R D";
    let scramble = "B' L' R2 U B2 R2 U' R2 F2 D2 R2 L' F R2 F' D B D";
    // let scramble = "";

    // Scramble the Cube
    cube.apply_scramble(scramble);

    // Solve the cube using CFOP
    // Determine the color of the bottom and top face
    let bottom = cube.faces[1][4];
    let top = cube.faces[0][4];

    // Step 1: Solve the cross
    println!("-------------cross-------------");
    let start_time = Instant::now();
    // let mut cube_cross = solve_cross(&mut cube, &target);
    let cross_moves = solve_cross(&mut cube, &bottom);
    let cross_elapsed_time = start_time.elapsed();
    println!("{}", cross_moves.join(" "));
    println!("Number of Moves: {}", cross_moves.len());
    println!("Elapsed time: {:?}", cross_elapsed_time);
    
    // Step 2: Solve the first 2 layers
    println!("-------------F2L-------------");
    let start_time = Instant::now();
    // let mut cube_f2l = solve_f2l(&mut cube_cross, &target);
    let f2l_moves = solve_f2l(&mut cube, &bottom);
    let f2l_elapsed_time = start_time.elapsed();
    println!("{}", f2l_moves.join(" "));
    println!("Number of Moves: {}", f2l_moves.len());
    println!("Elapsed time: {:?}", f2l_elapsed_time);
    
    // Step 3: Solve OLL
    println!("-------------OLL-------------");
    let start_time = Instant::now();
    let oll_moves = solve_oll(&mut cube, &top);
    let oll_elapsed_time = start_time.elapsed();
    println!("{}", oll_moves.join(" "));
    println!("Number of Moves: {}", oll_moves.len());
    println!("Elapsed time: {:?}", oll_elapsed_time);
    
    // Step 4: Solve PLL
    println!("-------------PLL-------------");
    let start_time = Instant::now();
    let pll_moves = solve_pll(&mut cube);
    let pll_elapsed_time = start_time.elapsed();
    println!("{}", pll_moves.join(" "));
    println!("Number of Moves: {}", pll_moves.len());
    println!("Elapsed time: {:?}", pll_elapsed_time);
    
    // Step 5: Total Moves
    println!("-------------Total-------------");
    let mut total_moves = Vec::new();
    total_moves.extend(cross_moves.clone());
    total_moves.extend(f2l_moves.clone());
    total_moves.extend(oll_moves.clone());
    total_moves.extend(pll_moves.clone());
    let total_moves_cleaned = cleanup_moves(total_moves);
    println!("{}", total_moves_cleaned.join(" "));
    println!("Number of Moves: {}", total_moves_cleaned.len());

    // Output data into excel file
    let _ = output_data((scramble, scramble.split(" ").collect::<Vec<&str>>().len()), 
    (cross_moves.clone(), cross_moves.len(), cross_elapsed_time),
    (f2l_moves.clone(), f2l_moves.len(), f2l_elapsed_time),
    (oll_moves.clone(), oll_moves.len(), oll_elapsed_time),
    (pll_moves.clone(), pll_moves.len(), pll_elapsed_time),
    (total_moves_cleaned.clone(), total_moves_cleaned.len(), cross_elapsed_time+f2l_elapsed_time+oll_elapsed_time+pll_elapsed_time));
    
    // // Visualize scrambled cube
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .run()

}
