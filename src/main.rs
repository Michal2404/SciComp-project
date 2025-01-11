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
use cfop::helper::cleanup_moves;

fn main() {
    // Create new instance of the Cube
    let mut cube = RubiksCube::new();
    // Define the scramble in the standard notation
    // let scramble = "L R U D";
    // let scramble = "R2 F R B D R F2 L F' D2 L U2 R B2 R2 F2 R2 U2 R' D2 L2";
    // let scramble = "R U B2 L D' R' U' B R U F2 B2 U' R2 D' R2 F2 U' L2 B2 U2";
    let scramble = "B U2 L' B L2 U' B2 U' F L U2 R2 D F2 U' L2 D R2 F2 B2 U2";
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
    let elapsed_time = start_time.elapsed();
    println!("{}", cross_moves.join(" "));
    println!("Number of Moves: {}", cross_moves.len());
    println!("Elapsed time: {:?}", elapsed_time);
    
    // Step 2: Solve the first 2 layers
    println!("-------------F2L-------------");
    let start_time = Instant::now();
    // let mut cube_f2l = solve_f2l(&mut cube_cross, &target);
    let f2l_moves = solve_f2l(&mut cube, &bottom);
    let elapsed_time = start_time.elapsed();
    println!("{}", f2l_moves.join(" "));
    println!("Number of Moves: {}", f2l_moves.len());
    println!("Elapsed time: {:?}", elapsed_time);
    
    // Step 3: Solve OLL
    println!("-------------OLL-------------");
    let start_time = Instant::now();
    let oll_moves = solve_oll(&mut cube, &top);
    let elapsed_time = start_time.elapsed();
    println!("{}", oll_moves.join(" "));
    println!("Number of Moves: {}", oll_moves.len());
    println!("Elapsed time: {:?}", elapsed_time);
    
    // Step 4: Solve PLL
    println!("-------------PLL-------------");
    let start_time = Instant::now();
    let pll_moves = solve_pll(&mut cube);
    let elapsed_time = start_time.elapsed();
    println!("{}", pll_moves.join(" "));
    println!("Number of Moves: {}", pll_moves.len());
    println!("Elapsed time: {:?}", elapsed_time);
    
    // Determine total moves
    println!("-------------Total-------------");
    let mut total_moves = Vec::new();
    total_moves.extend(cross_moves.clone());
    total_moves.extend(f2l_moves.clone());
    total_moves.extend(oll_moves.clone());
    total_moves.extend(pll_moves.clone());
    let total_moves_cleaned = cleanup_moves(total_moves);
    // println!("{} {} {} {}", cross_moves.join(" "), f2l_moves.join(" "), oll_moves.join(" "), pll_moves.join(" "));
    println!("{}", total_moves_cleaned.join(" "));
    println!("Number of Moves: {}", total_moves_cleaned.len());
    
    // // Visualize scrambled cube
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .run()

}
