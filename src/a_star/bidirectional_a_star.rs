// This file performs bi-directional a star search, which in theory cuts the time to search in half
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::helper::utils::*;
use crate::rubiks::cube::RubiksCube;

pub fn bidirectional_a_star_solver(scramble: &str, cube: &mut RubiksCube) -> Vec<String> {
    /*
    This function performs bidirectional a star search
     */
    // Open sets for forward and backward searches
    let mut open_set_f: BinaryHeap<Reverse<(usize, RubiksCube, String)>> = BinaryHeap::new();
    let mut open_set_b: BinaryHeap<Reverse<(usize, RubiksCube, String)>> = BinaryHeap::new();

    // Initialize the forward search
    open_set_f.push(Reverse((0, cube.clone(), "".to_string())));

    // Initialize the backward search
    let solved_cube = RubiksCube::new();
    open_set_b.push(Reverse((0, solved_cube.clone(), "".to_string())));

    // G-scores for both directions
    let mut g_score_f: HashMap<RubiksCube, usize> = HashMap::from([(cube.clone(), 0)]);
    let mut g_score_b: HashMap<RubiksCube, usize> = HashMap::from([(solved_cube.clone(), 0)]);

    // Estimated total cost (f-scores)
    let mut f_score_f: HashMap<RubiksCube, usize> =
        HashMap::from([(cube.clone(), heuristics(&cube))]);
    let mut f_score_b: HashMap<RubiksCube, usize> =
        HashMap::from([(solved_cube.clone(), heuristics(&solved_cube))]);

    // Tracking paths
    let mut came_from_f: HashMap<(usize, RubiksCube, String), (usize, RubiksCube, String)> =
        HashMap::new();
    let mut came_from_b: HashMap<(usize, RubiksCube, String), (usize, RubiksCube, String)> =
        HashMap::new();

    // Meeting point
    let mut meeting_point: Option<RubiksCube> = None;

    while !open_set_f.is_empty() && !open_set_b.is_empty() {
        // Expand forward search
        if let Some(Reverse(current_f)) = open_set_f.pop() {
            let mut current_cube_f = current_f.1.clone();

            // Check if the current forward node is in the backward closed set
            if g_score_b.contains_key(&current_cube_f) {
                meeting_point = Some(current_cube_f.clone());
                println!("went here");
                break;
            }

            // Generate neighbors for forward search
            // for (move_name, mut neighbor_cube) in current_cube_f.all_moves() {
            for move_name in current_cube_f.all_moves() {
                let mut neighbor_cube = current_cube_f.clone(); 
                neighbor_cube.apply_scramble(&move_name.0);

                let tentative_g_score =
                    g_score_f.get(&current_cube_f).unwrap_or(&usize::MAX) + 1;

                if tentative_g_score < *g_score_f.get(&neighbor_cube).unwrap_or(&usize::MAX) {
                    g_score_f.insert(neighbor_cube.clone(), tentative_g_score);
                    f_score_f.insert(
                        neighbor_cube.clone(),
                        tentative_g_score + heuristics(&neighbor_cube),
                    );

                    came_from_f.insert(
                        (
                            *f_score_f.get(&neighbor_cube).unwrap(),
                            neighbor_cube.clone(),
                            move_name.0.to_string().clone(),
                        ),
                        current_f.clone(),
                    );

                    open_set_f.push(Reverse((
                        *f_score_f.get(&neighbor_cube).unwrap(),
                        neighbor_cube.clone(),
                        move_name.0.to_string(),
                    )));
                }
            }
        }

        // Expand backward search
        if let Some(Reverse(current_b)) = open_set_b.pop() {
            let mut current_cube_b = current_b.1.clone();

            // Check if the current backward node is in the forward closed set
            if g_score_f.contains_key(&current_cube_b) {
                meeting_point = Some(current_cube_b.clone());
                break;
            }

            // Generate neighbors for backward search
            for move_name in current_cube_b.all_moves() {
                let mut neighbor_cube = current_cube_b.clone();
                neighbor_cube.apply_scramble(&move_name.0);

                let tentative_g_score =
                    g_score_b.get(&current_cube_b).unwrap_or(&usize::MAX) + 1;

                if tentative_g_score < *g_score_b.get(&neighbor_cube).unwrap_or(&usize::MAX) {
                    g_score_b.insert(neighbor_cube.clone(), tentative_g_score);
                    f_score_b.insert(
                        neighbor_cube.clone(),
                        tentative_g_score + heuristics(&neighbor_cube),
                    );

                    came_from_b.insert(
                        (
                            *f_score_b.get(&neighbor_cube).unwrap(),
                            neighbor_cube.clone(),
                            move_name.0.to_string().clone(),
                        ),
                        current_b.clone(),
                    );

                    open_set_b.push(Reverse((
                        *f_score_b.get(&neighbor_cube).unwrap(),
                        neighbor_cube.clone(),
                        move_name.0.to_string(),
                    )));
                }
            }
        }
    }

    // Reconstruct the path if a meeting point was found
    if let Some(meeting_cube) = meeting_point {
        let mut forward_path = vec![];
        let mut backward_path = vec![];

        // Reconstruct the forward path
        let mut current = (f_score_f[&meeting_cube], meeting_cube.clone(), "".to_string());
        while current.2 != "" {
            forward_path.push(current.2.clone());
            current = came_from_f[&current].clone();
        }

        // Reconstruct the backward path
        let mut current = (f_score_b[&meeting_cube], meeting_cube.clone(), "".to_string());
        while current.2 != "" {
            backward_path.push(current.2.clone());
            current = came_from_b[&current].clone();
        }

        backward_path.reverse();
        forward_path.append(&mut backward_path);

        let moves = forward_path
            .iter()
            .flat_map(|s| s.split_whitespace())
            .map(|s| s.to_string())
            .collect();

        return cleanup_moves(moves);
    }

    vec![] // Return an empty solution if no path is found
}

fn heuristics(cube: &RubiksCube) -> usize {
    /*
    This function creates heuristics to solve cube using a* algorithm
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
