// File to solve pll
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;
use crate::cfop::helper::*;

use std::collections::{HashMap, HashSet};
use std::fs;

pub fn solve_pll(cube: &mut RubiksCube) -> Vec<String>{
    /*
    This function solves the pll
     */
    // read the pll text file to determine which algorithm to use
    let map = read_file("src/cfop/pll_algorithm.txt");

    // initialize list that contains all the moves
    let mut output_list: Vec<String> = Vec::new();
    let mut y_list: Vec<String> = Vec::new();
    
    // loop through until we find a solution
    let mut solved = false;
    while !solved {
        // First look at the position of the target color
        let position2: Vec<(usize, usize)> = piece_location(cube, &cube.faces[2][4]);
        let position3: Vec<(usize, usize)> = piece_location(cube, &cube.faces[3][4]);
        let position4: Vec<(usize, usize)> = piece_location(cube, &cube.faces[4][4]);
        let position5: Vec<(usize, usize)> = piece_location(cube, &cube.faces[5][4]);

        // now loop through the keys and items in the hashmap
        for (algo, key) in &map {
            // check if the key matches the position
            let key_set2: HashSet<(usize, usize)> = key[0].iter().copied().collect();
            let key_set3: HashSet<(usize, usize)> = key[1].iter().copied().collect();
            let key_set4: HashSet<(usize, usize)> = key[2].iter().copied().collect();
            let key_set5: HashSet<(usize, usize)> = key[3].iter().copied().collect();
            // if we found the match, we will perform the algorithm, and add any values from y_list
            if contains_element(key_set2, position2.clone()) && contains_element(key_set3, position3.clone()) && contains_element(key_set4, position4.clone()) && contains_element(key_set5, position5.clone()) {
                // loop through y_list and add y
                for y in &y_list {
                    output_list.push(y.clone());
                    // cube.apply_scramble(y.as_str());
                }

                // now perform 
                output_list.push(algo.clone());
                cube.apply_scramble(algo.as_str());
                solved = true;
            }
        }
        // if solved is still false and cube hasn't made a full rotation yet, that means we must rotate the whole cube and try again
        if !solved && y_list.len() < 4 {
            y_list.push("y".to_string());
            cube.apply_scramble("y");
        }
        // if solved is still false and the cube has made a full rotation, we turn the top of cube and reset y_list
        else if !solved && y_list.len() == 4 {
            output_list.push("U".to_string());
            cube.apply_scramble("U");
            y_list = Vec::new();

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
    for i in 2..6 as usize {
        for j in 0..3 as usize {
            // if the edge has the target color, we return this
            if cube.faces[i][j] == *target {
                position.push((i, j));
            }
        }
    }
    return position
}

fn read_file(filename: &str) -> HashMap<String, Vec<Vec<(usize, usize)>>> {
    /*
    This function reads the file and outputs the contents in a string
     */
    // open file
    let contents = fs::read_to_string(filename)
        .expect("Failed to read the file.");

    // initialize hashmap that stores this information
    let mut algorithm_data = HashMap::new();

    // iterate through each line of the file
    for line in contents.lines() {
        // Iterate over each substring, split by ;, and attempt to parse as string
        let parts: Vec<&str> = line.split("; ").collect();
        
        // Skip lines starting with '%'
        if line.starts_with('%') || parts.len() != 5 {
            continue;
        }

        // we will perform different parsing techniques for different parts of the content
        let part0: Vec<(usize, usize)> = parse_vec_usize(parts[0]);
        let part1: Vec<(usize, usize)> = parse_vec_usize(parts[1]);
        let part2: Vec<(usize, usize)> = parse_vec_usize(parts[2]);
        let part3: Vec<(usize, usize)> = parse_vec_usize(parts[3]);
        let part4: String = parts[4].to_string();

        // update the algorithm data
        algorithm_data.insert(part4, vec![part0, part1, part2, part3]);

    }
    algorithm_data
}