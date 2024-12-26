// File to solve the cross in CFOP
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;
use std::collections::HashMap;
use std::collections::BinaryHeap;


pub fn solve_cross(cube: &RubiksCube) {
    /*
    This function solves the cross depending on what color we choose
    */
    // Step 1: Determine the color of the bottom face
    let target = cube.faces[1][4];

    // Step 2: Locate all edges with the target color
    let edges = find_edges_with_color(cube, &target);

    // Step 3: Determine location of center 
    let center_color = location_side_colors(cube, &target);

    // Determine location of target color
    let face_target = find_nested_index(cube, &target);

    // Determine priority queue
    let mut open_set: BinaryHeap<(usize, &RubiksCube, String)> = BinaryHeap::new();
    open_set.push((0, cube, "".to_string()));

    // Track state transitions
    let mut came_from: HashMap<(usize, &RubiksCube, String), (usize, &RubiksCube, String)> = HashMap::new();

    // Cost to reach a state
    let mut g_score: HashMap<&RubiksCube, usize> = HashMap::from([(cube, 0)]);
    // Estimated total cost
    let mut f_score: HashMap<&RubiksCube, usize> = HashMap::from([(cube, heuristics(cube))]);

    // println!("{:?}", edges);
    // println!("{:?}", center_color);
    
    // Step 4: Move the edges to the correct location
    // let mut current: (usize, &RubiksCube, String);
    while !open_set.is_empty() {
        let Some(current) = open_set.pop() else {panic!("something is wrong here")};
        // Clone the Rubik's Cube part of the tuple for reuse
        let mut current_cube = current.1.clone();
        // if we made the cross, we stop and return the moves
        if solved_state(&current_cube, &target) {
            let mut i = current.clone();
            while i.2 != ""{
                println!("{:?}", i.2);
                i = came_from[&i].clone();
            }
        }
    
        // we generate all possible moves
        // let mut current_cube = current.1.clone();
        for i in current_cube.all_moves() {
            let mut temp = current_cube.clone(); // Clone the cube for modifications
            temp.apply_scramble(i.0);
            // let neighbor = temp.apply_scramble(i.0);
    
            // Check or update tentative g_score
            let tentative_g_score = g_score.get(&temp).unwrap_or(&usize::MAX) + 1; // Default to max if not found
            if tentative_g_score < *g_score.get(&temp).unwrap_or(&usize::MAX) {
                // Insert into g_score and came_from with owned types
                g_score.insert(&temp, tentative_g_score);
                came_from.insert((tentative_g_score, &temp, i.0.to_string()), (current.0, &current_cube, current.2.clone()));
                open_set.push((tentative_g_score, &temp, i.0.to_string()));
            }
    
    
            // let tentative_g_score = g_score[&temp] + 1; // Assume uniform cost
        }

    }
        // ------------------we will move this outside the if statement once implementing while loop--------------------


}

fn solved_state(cube: &RubiksCube, target: &Color) -> bool {
    /*
    This function checks if the final cross has been made
     */
    // initialize boolean
    let mut check: bool = false;
    // first check the cross
    let face_target = find_nested_index(cube, target);
    let center = location_side_colors(cube, target);
    let list: Vec<usize> = vec![1, 3, 5, 7];
    
    for i in list{
        if cube.faces[face_target][i] == *target {
            check = true;
        }
        else {
            check = false;
            break
        }
    }

    // next the cross must align with face colors
    if check == true {
        for i in 0..center.len() {
            if cube.faces[center[i].0.0][4] == cube.faces[center[i].0.0][1]{
                check = true;
            }
            else {
                check = false;
                break
            }
        }
    }
        
        // // initialize vector which specifies conditions which satisifies the solved state
        // let mut state: Vec<((usize, usize), &Color)> = Vec::new();
        // state.push(((face_target, 1), target));
        // state.push(((face_target, 3), target));
        // state.push(((face_target, 5), target));
        // state.push(((face_target, 7), target));

    check



}
fn find_edges_with_color(cube: &RubiksCube, target: &Color) -> Vec<(usize, usize)> {
    /*
    This function finds the edges with the target colors
     */
    let mut edges: Vec<(usize, usize)> = Vec::new();
    // loop through all positions
    for i in 0..6 as usize {
        for j in 0..9 as usize {
            // if the edge has the target color, we return this
            if cube.faces[i][j] == *target && (j == 1 || j == 3 || j == 5 || j == 7) {
                // // find the color on the other side
                // let mut a: usize = 0;
                // let mut b: usize = 0;
                // if i == 2 && (j != 0 || j != 1) {
                //     a = 0
                // }
                edges.push((i, j));
            }
        }
    }
    return edges
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

fn heuristics(cube: &RubiksCube) -> usize {
    /*
    This function calculates the priority given a heuristics algorithm
     */
    let priority: usize = 0;
    priority
}
