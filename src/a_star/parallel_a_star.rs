use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    sync::{Arc, Mutex, RwLock},
};
use rayon::prelude::*;

use crate::helper::utils::*;
use crate::rubiks::cube::RubiksCube;

pub fn parallel_a_star_solver(scramble: &str, cube: &mut RubiksCube) -> Vec<String> {
    // Priority queue
    let open_set = Arc::new(Mutex::new(BinaryHeap::new()));
    open_set.lock().unwrap().push(Reverse((0, cube.clone(), "".to_string())));

    // Shared state maps
    let g_score = Arc::new(RwLock::new(HashMap::from([(cube.clone(), 0)])));
    let f_score = Arc::new(RwLock::new(HashMap::from([(cube.clone(), heuristics(&cube))])));
    let came_from: Arc<Mutex<HashMap<(usize, RubiksCube, String), (usize, RubiksCube, String)>>> = Arc::new(Mutex::new(HashMap::new()));

    // Output list
    let mut output_list: Vec<String> = Vec::new();

    while !open_set.lock().unwrap().is_empty() {
        let current = {
            let mut open_set = open_set.lock().unwrap();
            if let Some(Reverse(current)) = open_set.pop() {
                current
            } else {
                panic!("Failed to pop from open_set");
            }
        };

        let mut current_cube = current.1.clone();

        // If solved, reconstruct the path
        if current_cube == RubiksCube::new() {
            let mut i = current;
            let came_from = came_from.lock().unwrap();
            while i.2 != "" {
                output_list.push(i.2.clone());
                i = came_from[&i].clone();
            }
            output_list.reverse();
            let solve = output_list.join(" ");
            cube.apply_scramble(&solve);
            return cleanup_moves(
                output_list
                    .iter()
                    .flat_map(|s| s.split_whitespace())
                    .map(|s| s.to_string())
                    .collect(),
            );
        }

        // Generate all moves in parallel
        let all_moves = current_cube.all_moves();
        all_moves.par_iter().for_each(|(move_name, _)| {
            let mut temp_cube = current_cube.clone();
            temp_cube.apply_scramble(move_name);

            let tentative_g_score = {
                let g_score = g_score.read().unwrap();
                g_score.get(&current_cube).unwrap_or(&usize::MAX) + 1
            };

            let mut g_score_write = g_score.write().unwrap();
            if tentative_g_score < *g_score_write.get(&temp_cube).unwrap_or(&usize::MAX) {
                g_score_write.insert(temp_cube.clone(), tentative_g_score);

                let mut f_score_write = f_score.write().unwrap();
                f_score_write.insert(temp_cube.clone(), tentative_g_score + heuristics(&temp_cube));

                let mut came_from_write = came_from.lock().unwrap();
                came_from_write.insert(
                    (
                        *f_score_write.get(&temp_cube).unwrap(),
                        temp_cube.clone(),
                        move_name.to_string().clone(),
                    ),
                    current.clone(),
                );

                let mut open_set_write = open_set.lock().unwrap();
                open_set_write.push(Reverse((
                    *f_score_write.get(&temp_cube).unwrap(),
                    temp_cube.clone(),
                    move_name.to_string().clone(),
                )));
            }
        });
    }

    output_list
}

fn heuristics(cube: &RubiksCube) -> usize {
    /*
    Heuristics used for parallel programming
     */
    // Your heuristic function remains the same
    let solved_cube = RubiksCube::new();
    let mut count = 0;

    for i in 0..=2 {
        for j in 0..=2 {
            for k in 0..=2 {
                let local = global_to_local(i, j, k);
                for location in local {
                    if cube.faces[location.0][location.1]
                        != solved_cube.faces[location.0][location.1]
                    {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    count
}
