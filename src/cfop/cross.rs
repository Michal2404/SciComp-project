// File to solve the cross in CFOP
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;
use crate::helper::utils::*;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


pub fn solve_cross(cube: &mut RubiksCube, target: &Color) -> Vec<String>{
    /*
    This function solves the cross depending on what color we choose
    */

    // Determine priority queue
    let mut open_set: BinaryHeap<Reverse<(usize, RubiksCube, String)>> = BinaryHeap::new();
    open_set.push(Reverse((0, cube.clone(), "".to_string())));

    // Track state transitions
    let mut came_from: HashMap<(usize, RubiksCube, String), (usize, RubiksCube, String)> = HashMap::new();

    // Cost to reach a state
    let mut g_score: HashMap<RubiksCube, usize> = HashMap::from([(cube.clone(), 0)]);
    // Estimated total cost
    let mut f_score: HashMap<RubiksCube, usize> = HashMap::from([(cube.clone(), heuristics(cube, target))]);
    
    // Step 4: Move the edges to the correct location
    // we will append this to a list and print out the list
    let mut output_list: Vec<String> = Vec::new();
    while !open_set.is_empty() {
        let Some(Reverse(current)) = open_set.pop() else {panic!("something is wrong here")};
        // Clone the Rubik's Cube part of the tuple for reuse
        let mut current_cube = current.1.clone();
        // if we made the cross, we stop and return the moves
        if solved_state(&current_cube, target).0 {
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
            
            // println!("{}", output_list.join(" "));
            let moves = output_list.iter().flat_map(|s| s.split_whitespace()).map(|s| s.to_string()).collect();
            return cleanup_moves(moves)
        }
        
        // we generate all possible moves
        for i in current_cube.all_moves() {
            let mut temp = current_cube.clone(); // Clone the cube for modifications
            temp.apply_scramble(i.0);
            // let neighbor = temp.apply_scramble(i.0);
            
            // Check or update tentative g_score
            // let tentative_g_score = *g_score.get(&temp).unwrap_or(&usize::MAX); // Default to max if not found
            let tentative_g_score = *g_score.get(&current_cube).unwrap_or(&usize::MAX) + 1; // Default to max if not found
            if tentative_g_score < *g_score.get(&temp).unwrap_or(&usize::MAX) {
                // Insert into g_score and came_from with owned types
                g_score.insert(temp.clone(), tentative_g_score);
                f_score.insert(temp.clone(), tentative_g_score + heuristics(&temp, &target));
                came_from.insert((*f_score.get(&temp).unwrap(), temp.clone(), i.0.to_string()), (current.0, current_cube.clone(), current.2.clone()));
                open_set.push(Reverse((*f_score.get(&temp).unwrap(), temp.clone(), i.0.to_string())));
            }

            // println!("{:?}", heuristics(&temp, &target));
            
            
            // println!("{:?}", tentative_g_score);
            // println!("{:?}", *g_score.get(&temp).unwrap_or(&usize::MAX));
            // let tentative_g_score = g_score[&temp] + 1; // Assume uniform cost
        }

        
    }

    return output_list
    
    // return cube.clone();
}

fn solved_state(cube: &RubiksCube, target: &Color) -> (bool, usize) {
    /*
    This function checks if the final cross has been made, and gives the number of edges that are in the wrong spot
     */
    // initialize boolean
    let mut check: bool = true;
    // initialize variables here
    let face_target = find_nested_index(cube, target);
    let center = location_side_colors(cube, target);
    let list: Vec<usize> = vec![1, 7, 5, 3];
    let mut misplaced: usize = 4;

    // first check the cross
    for (index, i) in list.iter().enumerate() {
        // check if the edge face is on the cross and if its center color matches the edge piece
        if cube.faces[face_target][*i] == *target && cube.faces[center[index].0.0][4] == cube.faces[center[index].0.0][7] {
            misplaced -= 1;
        }
        else {
            check = false;
        }
    }
    
    (check, misplaced)
}

fn location_side_colors(cube: &RubiksCube, target: &Color) -> Vec<((usize, usize), Color)> {
    /*
    This function determines the location of all side color centers
    */
    let mut center: Vec<((usize, usize), Color)> = Vec::new();

    // loop through all positions
    let face_target = find_nested_index(cube, target);
    for i in 0..6 as usize {
        // determine which one is the opposite from target color, and we will not include that
        let face_opposite = opposite_face(face_target);
        if i == face_opposite || i == face_target {
            continue
        }
        center.push(((i, 4), cube.faces[i][4]))
    }

    // println!("{:?}", center);
    center
}

fn heuristics(cube: &RubiksCube, target:&Color) -> usize {
    /*
    This function calculates the priority given a heuristics algorithm
     */
    // we will calculate heuristics based on number of misplaced edges
    let (_, misplaced) = solved_state(cube, target);
    misplaced
}


// ------------------------ Helper Functions ------------------------
fn find_nested_index(cube: &RubiksCube, target: &Color) -> usize {
    /*
    Helper function that determines the location of color
     */
    let mut index: usize = 0;

    // loop through the outer layer
    for (outer_idx, inner_vec) in cube.faces.iter().enumerate() {
        // loop through the inner layer
        for (inner_idx, &value) in inner_vec.iter().enumerate() {
            if value == *target && inner_idx == 4 {
                // inner index must be 4
                index = outer_idx;
            }
        }
    }
    index
}

fn opposite_face(face_num: usize) -> usize {
    /*
    Helper function that determines the opposite face
     */
    let opposite: usize = match face_num {
        0 => 1,
        1 => 0,
        2 => 3,
        3 => 2,
        4 => 5,
        5 => 4,
        _ => panic!("number must be between 0 and 5"),
    };
    opposite
}

