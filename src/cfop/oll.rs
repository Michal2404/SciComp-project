// File to solve OLL
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;
use crate::cfop::helper::*;

use std::collections::{HashMap, HashSet};
use std::fs;

pub fn solve_oll(cube: &mut RubiksCube, target: &Color) -> Vec<String> {
    /*
    This function solves the oll
     */
    // read the oll text file to determine which algorithm to use
    let map: HashMap<Vec<(usize, usize)>, String> = read_file("src/cfop/oll_algorithm.txt");
    // println!("{:?}", position);
    // println!("{:?}", map);
    
    // initialize list that contains all the moves
    let mut output_list: Vec<String> = Vec::new();
    
    // loop through until we find a solution
    let mut solved = false;
    while !solved {
        // First look at the position of the target color
        let position: Vec<(usize, usize)> = piece_location(cube, target);
        // now loop through the keys and items in the hashmap
        for (key, algo) in &map {
            // check if the key matches the position
            let key_set: HashSet<(usize, usize)> = key.iter().copied().collect();
            if contains_element(key_set, position.clone()) {
                output_list.push(algo.clone());
                cube.apply_scramble(algo.as_str());
                solved = true;
            }
        }
        // if solved is still false, that means we must rotate the top layer and try again
        if !solved {
            output_list.push("U".to_string());
            cube.apply_scramble("U");
        }

    }

    // once all done, we will print out the list
    println!("{}", output_list.join(" "));
    return output_list

}

fn piece_location(cube: &RubiksCube, target: &Color) -> Vec<(usize, usize)> {
    /*
    This function finds the location of where the target color is on
     */
    let mut position: Vec<(usize, usize)> = Vec::new();
    // loop through all positions
    for i in 0..6 as usize {
        for j in 0..9 as usize {
            // if the edge has the target color, we return this
            if cube.faces[i][j] == *target {
                position.push((i, j));
            }
        }
    }
    return position
}

fn read_file(filename: &str) -> HashMap<Vec<(usize, usize)>, String> {
    /*
    This function reads the file and outputs the contents in a string
     */
    // open file
    let contents = fs::read_to_string(filename)
        .expect("Failed to read the file.");

    // initialize hashmap that stores this information
    let mut algorithm_data: HashMap<Vec<(usize, usize)>, String> = HashMap::new();

    // iterate through each line of the file
    for line in contents.lines() {
        // Iterate over each substring, split by ;, and attempt to parse as string
        let parts: Vec<&str> = line.split("; ").collect();
        
        // Skip lines starting with '%'
        if line.starts_with('%') || parts.len() != 2 {
            continue;
        }

        // we will perform different parsing techniques for different parts of the content
        let part0: Vec<(usize, usize)> = parse_vec_usize(parts[0]);
        let part1: String = parts[1].to_string();

        // update the algorithm data
        algorithm_data.insert(part0, part1);

    }
    algorithm_data
}