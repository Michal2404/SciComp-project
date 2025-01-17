// This function performs the normal a_star algorithm to solve the cube
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, time::Instant};

use crate::{helper::utils::*, rubiks::color::Color};
use crate::rubiks::cube::RubiksCube;



pub fn a_star_solver(scramble: &str, cube: &mut RubiksCube) -> Vec<String> {
    /*
    This function using the a star search
     */
    // Determine priority queue
    let mut open_set: BinaryHeap<Reverse<(usize, RubiksCube, String)>> = BinaryHeap::new();
    open_set.push(Reverse((0, cube.clone(), "".to_string())));

    // Track state transitions
    let mut came_from: HashMap<(usize, RubiksCube, String), (usize, RubiksCube, String)> = HashMap::new();

    // Cost to reach a state
    let mut g_score: HashMap<RubiksCube, usize> = HashMap::from([(cube.clone(), 0)]);
    // Estimated total cost
    let mut f_score: HashMap<RubiksCube, usize> = HashMap::from([(cube.clone(), heuristics(&cube))]);
    
    // Step 4: Move the edges to the correct location
    // we will append this to a list and print out the list
    let mut output_list: Vec<String> = Vec::new();
    while !open_set.is_empty() {
        let Some(Reverse(current)) = open_set.pop() else {panic!("something is wrong here")};
        // Clone the Rubik's Cube part of the tuple for reuse
        let mut current_cube = current.1.clone();
        // if we solved the cube, we stop and return the moves
        if current_cube == RubiksCube::new() {
            let mut i: (usize, RubiksCube, String) = current.clone();
            while i.2 != ""{
                output_list.push(i.2.clone());
                i = came_from[&i].clone();
            }

            // reverse the list
            output_list.reverse();
            
            // apply the solution to the cube
            let solve = output_list.join(" ");
            cube.apply_scramble(solve.as_str());
            
            let moves = output_list.iter().flat_map(|s| s.split_whitespace()).map(|s| s.to_string()).collect();
            return cleanup_moves(moves)
        }
        
        // we generate all possible moves
        for i in current_cube.all_moves() {
            let mut temp = current_cube.clone(); // Clone the cube for modifications
            temp.apply_scramble(i.0);
            
            // Check or update tentative g_score
            // let tentative_g_score = *g_score.get(&temp).unwrap_or(&usize::MAX); // Default to max if not found
            let tentative_g_score = *g_score.get(&current_cube).unwrap_or(&usize::MAX) + 1; // Default to max if not found
            if tentative_g_score < *g_score.get(&temp).unwrap_or(&usize::MAX) {
                // Insert into g_score and came_from with owned types
                g_score.insert(temp.clone(), tentative_g_score);
                f_score.insert(temp.clone(), tentative_g_score + heuristics(&temp));
                came_from.insert((*f_score.get(&temp).unwrap(), temp.clone(), i.0.to_string()), (current.0, current_cube.clone(), current.2.clone()));
                open_set.push(Reverse((*f_score.get(&temp).unwrap(), temp.clone(), i.0.to_string())));
            }

        }

    }
    return output_list
}

fn heuristics(cube: &RubiksCube) -> usize {
    /*
    This function creates heuristics to solve cube using a* algorithm
    Heuristic:
        Determines the number of pieces that arent solved
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

                // next we determine if all faces are correct
                for location in local {
                    // if one face is incorrect, we add 1 to the count and break the for loop
                    if cube.faces[location.0][location.1] != solved_cube.faces[location.0][location.1] {
                        count += 1;
                        break
                    }
                }
            }
        }
    }

    // finally return the count
    count

}

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