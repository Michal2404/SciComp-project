// This function performs the normal a_star algorithm to solve the cube
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, time::{Duration, Instant}};

use crate::helper::utils::*;
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;

use umya_spreadsheet::*;

pub fn a_star_solver(scramble: &str, cube: &mut RubiksCube) -> Vec<String> {
    /*
    This function solves the cube using a star, and prints out necessary data
     */
    // keep track of time
    let start_time = Instant::now();
    // solve using a star
    let moves_cleaned = solve(cube);
    let elapsed_time = start_time.elapsed();
    // Print results here
    println!("-------------A*-------------");
    println!("{}", moves_cleaned.join(" "));
    println!("Number of Moves: {}", moves_cleaned.len());
    println!("Elapsed time: {:?}", elapsed_time);


    // output data
    // let data = (moves_cleaned.clone(), moves_cleaned.len(), elapsed_time);
    // let _ = output_data((scramble, scramble.split(" ").collect::<Vec<&str>>().len()), data);

    moves_cleaned

}

fn output_data(scramble: (&str, usize),
                data: (Vec<String>, usize, Duration)
                            ) -> Result<(), Box<dyn std::error::Error>> {
    /*
    This function outputs data into a excel file
        */
    // Open the Excel file
    let path = "src/a_star/a_star_algorithm.xlsx";
    let mut workbook = reader::xlsx::read(std::path::Path::new(path)).unwrap();

    // Specify the sheet name to read
    // Step 2: Access a specific worksheet
    let sheet = workbook.get_sheet_by_name_mut("raw data").unwrap();

    // Iterate over rows and check for non-empty rows
    // for (index, row) in sheet.get_row_collection().enumerate() {
    for row_num in 1..=u32::MAX {
        let default = Cell::default();
        let first_cell = sheet.get_cell_by_column_and_row(1, row_num).unwrap_or(&default);
        // Get the first cell of the row (column A)
        // Check if the cell's value matches the target string
        if first_cell.get_value() == scramble.0 {
            return Ok(())
        }
        if !first_cell.get_value().is_empty() {
            continue
        }
        // If it doesn't exist, that means we add values to it
        else {
            // now we add the values into the excel sheet
            // Define the cells and their new values (row, column, value)
            let updates = vec![
                (row_num, 1, scramble.0.to_string()),   // Row _, Column 1
                (row_num, 2, scramble.1.to_string()),    // Row _, Column 2
                (row_num, 3, data.0.join(" ")),// Row _, Column 3
                (row_num, 4, data.1.to_string()),// Row _, Column 4
                (row_num, 5, data.2.as_micros().to_string()),// Row _, Column 5
                ];
                
                // Loop through each update and apply it
                for (row, col, value) in updates {
                    assert!(col >= 1, "Column number starts from 1.");
                    sheet.get_cell_by_column_and_row_mut(col, row).set_value(value);
                }
                
                // // Open the same file and overwrite the original content
                let _ = writer::xlsx::write(&workbook, std::path::Path::new(path));
                println!("went here");
                // finally return
                return Ok(())
                
        }
    }
    Ok(())               
}

fn solve(cube: &mut RubiksCube) -> Vec<String> {
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
//     Determines number of moves needed to solve each individual piece
//     */
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
     */
    // initialize the distance
    let mut distance_edge = 0;
    let mut distance_corner = 0;
    let mut position = true;
    let mut orientation = true;

    // loop through each position on the global scale
    for i in 0..=2 {
        for j in 0..=2 {
            for k in 0..=2 {
                // skip the center
                if i == 1 && j == 1 && k == 1 {
                    continue
                }
                // first we convert this global scale into local
                let local = global_to_local(i, j, k);

                let mut target_location = vec![1, 1, 1];

                // determine the color of each side of the piece
                for vector in local.clone() {
                    // determine color
                    let color = cube.faces[vector.0][vector.1];
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
                
                // check if the piece is on the target location
                if target_location != vec![i, j, k] {
                    position = false;
                } else {
                    position = true;
                }
                
                // now check if this is a corner or edge
                // if this is a corner
                if local.clone().len() == 3 {
                    orientation = corner_orientation(cube, local.clone());
                    // now we determine the distance
                    if position && orientation {
                        distance_corner += 0;
                    } else if !position && orientation {
                        distance_corner += 1;
                    } else if position && !orientation {
                        distance_corner += 2;
                    } else {
                        distance_corner += 2;
                    }

                }
                // if this is an edge
                else if local.clone().len() == 2 {
                    orientation = edge_orientation(cube, local.clone());
                    if position && orientation {
                        distance_edge += 0;
                    } else if !position && orientation {
                        distance_edge += 1;
                    } else if !position && !orientation {
                        distance_edge += 2;
                    } else {
                        distance_edge += 3;
                    }
                }
                // if its the center, we skip
                else {
                    continue
                }
            
            }
        }
    }
    
    // distance_edge = distance_edge / 4;
    // distance_corner = distance_corner / 4;
    
    // println!("distance edge: {}", distance_edge);
    // println!("distance corner: {}", distance_corner);
    // finally return the count
    // distance_edge.max(distance_corner)
    // std::cmp::max(distance_edge, distance_corner)
    (distance_corner+distance_edge)/5

}

fn corner_orientation(cube: &RubiksCube, local: Vec<(usize, usize)>) -> bool {
    /*
    This function determines the orientation of the corner pieces
     */
    // determine the color of each side of the piece
    for vector in local.clone() {
        // determine color
        let color = cube.faces[vector.0][vector.1];
        // determine if white/yellow is in the correct position
        // check if white is on the top
        if (color == Color::W || color == Color::Y) && (vector.0 == 0 || vector.0 == 1) {
            return true
        } else { continue }
    }
    false
}

fn edge_orientation(cube: &RubiksCube, local: Vec<(usize, usize)>) -> bool {
    /*
    This function determines the orientation of the corner pieces
     */
    // initialize the faces
    // println!("local: {:?}", local);
    let mut face1 = local.clone()[0];
    let mut face2 = local.clone()[1];

    // check if the edge is in the correct position
    for _ in local.clone() {
        // determine color
        let color1 = cube.faces[face1.0][face1.1];
        let color2 = cube.faces[face2.0][face2.1];
        // look at top/bottom faces        
        if face1.0 == 0 || face1.0 == 1 || face1.0 == 2 || face1.0 == 3 {
            // if orange/red is on the top/bottom
            if color1 == Color::O || color1 == Color::R {
                return false
            }
            // if green/blue is on the top/bottom, need to check other side
            else if color1 == Color::G || color1 == Color::B {
                if color2 == Color::W || color2 == Color::Y {
                    return false
                }
            }
        // otherwise we flip the faces
        else {
            let temp = face1.clone();
            face1 = face2.clone();
            face2 = temp.clone();
        }

        }

    }
    true
}