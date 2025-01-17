// This function performs the iterative deep a star (IDA*) algorithm to solve the cube
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, time::Instant};

use crate::{helper::utils::*, rubiks::color::Color};
use crate::rubiks::cube::RubiksCube;

fn search(
    cube: &mut RubiksCube,
    g: usize, // Cost to reach this state
    threshold: usize, // Current threshold
    path: &mut Vec<String>, // Path of moves leading to this state
    visited: &mut HashSet<RubiksCube>, // Visited states
) -> Result<Vec<String>, usize> {
    /*
    This function performs IDA* search
     */
    let f = g + heuristics(&cube);

    if f > threshold {
        return Err(f); // Return the new threshold
    }

    if cube.is_solved() {
        return Ok(path.clone()); // Return the solution path
    }

    visited.insert(cube.clone());
    let mut min_threshold = usize::MAX;

    for i in cube.all_moves() {// Iterate over possible moves
        let mut next_cube = cube.clone();
        next_cube.apply_scramble(i.0);

        if visited.contains(&next_cube) {
            continue; // Skip visited states
        }

        path.push(i.0.to_string());
        match search(&mut next_cube, g + 1, threshold, path, visited) {
            Ok(solution) => return Ok(solution),
            Err(new_threshold) => min_threshold = min_threshold.min(new_threshold),
        }
        path.pop();
    }

    visited.remove(&cube);
    Err(min_threshold) // Return the smallest threshold that exceeded
}


pub fn ida_star_solver(scramble: &str, cube: &mut RubiksCube) -> Vec<String> {
    /*
    This function using the a star search
     */
    let mut threshold = heuristics(cube); // Initial threshold is the heuristic of the start state

    let mut count = 0;
    loop {
        let mut visited = HashSet::new();
        match search(cube, count, threshold, &mut Vec::new(), &mut visited) {
            Ok(solution) => return solution,
            Err(new_threshold) if new_threshold == usize::MAX => return Vec::new(), // No solution found
            Err(new_threshold) => threshold = new_threshold, // Increase threshold
        }
    }

}

// fn heuristics(cube: &RubiksCube) -> usize {
//     /*
//     This function creates heuristics to solve cube using a* algorithm
//     Heuristic:
//         Determines the number of pieces that arent solved
//      */
//     // initialize the solved rubiks cube
//     let solved_cube = RubiksCube::new();

//     // initialize count
//     let mut count = 0;
//     // check the number of faces that are already cube solved
//     // loop through each position on the global scale
//     for i in 0..=2 {
//         for j in 0..=2 {
//             for k in 0..=2 {
//                 // first we convert this global scale into local
//                 let local = global_to_local(i, j, k);

//                 // next we determine if all faces are correct
//                 for location in local {
//                     // if one face is incorrect, we add 1 to the count and break the for loop
//                     if cube.faces[location.0][location.1] != solved_cube.faces[location.0][location.1] {
//                         count += 1;
//                         break
//                     }
//                 }
//             }
//         }
//     }

//     // finally return the count
//     count

// }

// fn heuristics(cube: &RubiksCube) -> usize {
//     /*
//     This function creates heuristics to solve cube using a* algorithm
//     Heuristic:
//         Determines number of moves needed to solve each individual piece
//      */
//     // initialize the solved rubiks cube
//     let solved_cube = RubiksCube::new();

//     // initialize count
//     let mut count = 0;
//     // check the number of faces that are already cube solved
//     // loop through each position on the global scale
//     for i in 0..=2 {
//         for j in 0..=2 {
//             for k in 0..=2 {
//                 // first we convert this global scale into local
//                 let local = global_to_local(i, j, k);
//                 let mut target_location = vec![1, 1, 1];

//                 // determine the color of each side of the piece
//                 for vector in local.clone() {
//                     // determine color
//                     let color = cube.faces[vector.0][vector.1];
//                     // println!("color: {:?}", color);
//                     // find location of center color
//                     match color {
//                         Color::W => target_location[2] = 2,
//                         Color::Y => target_location[2] = 0,
//                         Color::G => target_location[1] = 0,
//                         Color::B => target_location[1] = 2,
//                         Color::R => target_location[0] = 2,
//                         Color::O => target_location[0] = 0,
//                     };
//                 }

//                 // next we determine how far the piece is to its final destination
//                 let position_moves_0 = if i < target_location[0] {target_location[0] - i} else {i - target_location[0]};
//                 let position_moves_1 = if j < target_location[1] {target_location[1] - j} else {j - target_location[1]};
//                 let position_moves_2 = if k < target_location[2] {target_location[2] - k} else {k - target_location[2]};
//                 let mut position_moves = position_moves_0 + position_moves_1 + position_moves_2;
//                 let mut orientation_moves = 0;
//                 // check if orientation is off eventhough its in the same location
//                 if position_moves == 0 {
//                     // check with local
//                     for face in local.clone() {
//                         if cube.faces[face.0][face.1] != solved_cube.faces[face.0][face.1] {
//                             // add 1 move (1 move == 2 here)
//                             orientation_moves = 2;
//                             break

//                         }
//                     } 
//                 }
//                 // println!("target location: {:?}", target_location);
//                 // println!("current location: {:?}", (i, j, k));
//                 // println!("position moves: {}", position_moves);
//                 // if this was an edge, we multiply answer by 2
//                 if local.clone().len() == 2 {
//                     position_moves = position_moves * 2;
//                     orientation_moves = orientation_moves * 3;
//                 }

//                 // update count
//                 count += position_moves + orientation_moves;
                
//             }
//         }
//     }

//     // finally return the count
//     count

// }

fn heuristics(cube: &RubiksCube) -> usize {
    /*
    This function creates heuristics to solve cube using a* algorithm
    Heuristic:
        Determines number of moves needed to solve each individual piece
     */
    // initialize the solved rubiks cube
    let solved_cube = RubiksCube::new();

    // initialize count
    let mut count = 0;
    // check the number of faces that are already cube solved
    // loop through each position on the global scale
    for i in 0..=2 {
        for j in 0..=2 {
            for k in 0..=2 {
                // first we convert this global scale into local
                let local = global_to_local(i, j, k);
                
                // determine the orientation of the cubie
                // determine the color of each side of the piece
                let mut target_location = vec![1, 1, 1];
                for vector in local.clone() {
                    // determine color
                    let color = cube.faces[vector.0][vector.1];
                    // println!("color: {:?}", color);
                    // find location of center color
                    match color {
                        Color::W => target_location[2] = 2,
                        Color::Y => target_location[2] = 0,
                        Color::G => target_location[1] = 0,
                        Color::B => target_location[1] = 2,
                        Color::R => target_location[0] = 2,
                        Color::O => target_location[0] = 0,
                    };
                }

                
            }
        }
    }

    // finally return the count
    count

}
