// File to solve the first two layers in CFOP
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;
use crate::helper::utils::*;

use std::fs;
use std::collections::HashMap;

pub fn solve_f2l(cube: &mut RubiksCube, target: &Color) -> Vec<String> {
    /*
    This function solves the f2l
     */

    // first determine the target location for corner and edges
    let target_edge_locations: HashMap<Vec<Color>, Vec<(usize, usize)>> = HashMap::from([
        (vec![Color::B, Color::R], vec![(3, 3), (4, 5)]),
        (vec![Color::G, Color::R], vec![(2, 5), (4, 3)]),
        (vec![Color::G, Color::O], vec![(2, 3), (5, 5)]),
        (vec![Color::B, Color::O], vec![(3, 5), (5, 3)]),
    ]);
    let target_corner_locations: HashMap<Vec<Color>, Vec<(usize, usize)>> = HashMap::from([
        (vec![Color::B, Color::R], vec![(3, 6), (4, 8)]),
        (vec![Color::G, Color::R], vec![(2, 8), (4, 6)]),
        (vec![Color::G, Color::O], vec![(2, 6), (5, 8)]),
        (vec![Color::B, Color::O], vec![(3, 8), (5, 6)]),
    ]);

    // initialize vector that stores all the colors
    let mut colors = vec![vec![Color::B, Color::R], vec![Color::G, Color::R], vec![Color::G, Color::O], vec![Color::B, Color::O]];

    // now we need to know where the pieces should end up in
    let map: HashMap<Vec<Color>, Vec<Algorithm>> = read_file("src/cfop/f2l_algorithm.txt");

    // initialize list that contains all the moves
    let mut output_list: Vec<String> = Vec::new();
    // initialize number of U moves 
    let mut count: usize = 0;

    // loop through each color and try to find algorithm that solves f2l
    while !colors.is_empty() {
        // initialize if any color solved
        let mut color_solved = false;
        // first loop through each color in the vector
        for color in colors.clone() {
            // find corner and edge piece locations
            let corner = corner_piece_location(cube, target, &color[0], &color[1]);
            let edge = edge_piece_location(cube, &color[0], &color[1]);
            
            // check if they are already solved and if it is, we move onto the next color
            if corner == *target_corner_locations.get(&color).unwrap() && edge == *target_edge_locations.get(&color).unwrap() {
                let index = colors.iter().position(|x| *x == *color).unwrap();
                colors.remove(index);
                continue
            }
            
            // otherwise, we determine if we can run any other algorithms on it
            for algo in map.get(&color).unwrap() {
                // if we found the algorithm perform it
                if algo.corner == corner && algo.edge == edge {
                    output_list.push(algo.moves.clone());
                    cube.apply_scramble(algo.moves.as_str());
                    // remove the color from vector
                    let index = colors.iter().position(|x| *x == *color).unwrap();
                    colors.remove(index);
                    // we indicate for this loop a color has been solved
                    color_solved = true;
                    // reset count
                    count = 0;
                    
                }
            }
            
        }
        // if the colors is still not empty and we haven't solved a color this loop, that means we have to make a random move and try it again
        if !colors.is_empty() && !color_solved {
            count += 1;
            output_list.push("U".to_string());
            cube.apply_scramble("U");
        }
        
        // TODO: for niche situations where none of this works, we need to locate this troubled piece and convert it into a state where we can perform algorithm
        // if count is 4, that means we did a full loop without finding algorithm, therefore we must take trouble piece out of slot
        if count == 4 {
            // locate troubled piece
            let corner = corner_piece_location(cube, target, &colors[0][0], &colors[0][1]);
            let edge = edge_piece_location(cube, &colors[0][0], &colors[0][1]);
            
            // convert local to global
            let (x_corner, y_corner, z_corner) = local_to_global(corner[0].0, corner[0].1);
            let (x_edge, y_edge, z_edge) = local_to_global(edge[0].0, edge[0].1);

            // check location of both corner and edge
            // slot 1
            if (x_corner == 2 && y_corner == 0 && z_corner == 0) || (x_edge == 2 && y_edge == 0 && z_edge == 1) {
                // we perform U action if either corner or edge piece is in replacement spot
                if (x_corner == 0 && y_corner == 2 && z_corner == 2) || (x_edge == 1 && y_edge == 2 && z_edge == 2) {
                    output_list.push("U".to_string());
                    cube.apply_scramble("U");
                }
                // now we can safely take out the troubled piece
                output_list.push("R U R'".to_string());
                cube.apply_scramble("R U R'");
            }
            // slot 2
            if (x_corner == 0 && y_corner == 0 && z_corner == 0) || (x_edge == 0 && y_edge == 0 && z_edge == 1) {
                // we perform U action if either corner or edge piece is in replacement spot
                if (x_corner == 2 && y_corner == 2 && z_corner == 2) || (x_edge == 1 && y_edge == 2 && z_edge == 2) {
                    output_list.push("U".to_string());
                    cube.apply_scramble("U");
                }
                // now we can safely take out the troubled piece
                output_list.push("L' U' L".to_string());
                cube.apply_scramble("L' U' L");
            }
            // slot 3
            if (x_corner == 0 && y_corner == 2 && z_corner == 0) || (x_edge == 0 && y_edge == 2 && z_edge == 1) {
                // we perform U action if either corner or edge piece is in replacement spot
                if (x_corner == 2 && y_corner == 2 && z_corner == 2) || (x_edge == 1 && y_edge == 2 && z_edge == 2) {
                    output_list.push("U".to_string());
                    cube.apply_scramble("U");
                }
                // now we can safely take out the troubled piece
                output_list.push("L U' L'".to_string());
                cube.apply_scramble("L U' L'");
            }
            // slot 4
            if (x_corner == 2 && y_corner == 2 && z_corner == 0) || (x_edge == 2 && y_edge == 2 && z_edge == 1) {
                // we perform U action if either corner or edge piece is in replacement spot
                if (x_corner == 0 && y_corner == 2 && z_corner == 2) || (x_edge == 1 && y_edge == 2 && z_edge == 2) {
                    output_list.push("U".to_string());
                    cube.apply_scramble("U");
                }
                // now we can safely take out the troubled piece
                output_list.push("R' U R".to_string());
                cube.apply_scramble("R' U R");
            }
            // update count
            count = 0;
        }
    }
    // once all done, we will return out the list
    let moves = output_list.iter().flat_map(|s| s.split_whitespace()).map(|s| s.to_string()).collect();

    cleanup_moves(moves)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Algorithm {
    corner: Vec<(usize, usize)>,
    edge: Vec<(usize, usize)>,
    moves: String
}

fn read_file(filename: &str) -> HashMap<Vec<Color>, Vec<Algorithm>> {
    /*
    This function reads the file and outputs the contents in a string
     */
    // open file
    let contents = fs::read_to_string(filename)
        .expect("Failed to read the file.");

    // initialize hashmap that stores this information
    let mut algorithm_data: HashMap<Vec<Color>, Vec<Algorithm>> = HashMap::new();

    // iterate through each line of the file
        for line in contents.lines() {
        // Iterate over each substring, split by ;, and attempt to parse as string
        let parts: Vec<&str> = line.split("; ").collect();

        // Skip lines starting with '%'
        if line.starts_with('%') || parts.len() != 4 {
            continue;
        }

        // we will perform different parsing techniques for different parts of the content
        let part1: Vec<(usize, usize)> = parse_vec_usize(parts[1]);
        let part2: Vec<(usize, usize)> = parse_vec_usize(parts[2]);
        
        // we will get the first string as colors
        let colors: Vec<Color> = parse_vec_color(parts[0]);

        // get the remaining parts into a struct
        let list = Algorithm {
            corner: part1,
            edge: part2,
            moves: parts[3].to_string()
        };

        // add this into the hashmap
        algorithm_data.entry(colors).or_default().push(list);

    }
    algorithm_data
}

fn edge_piece_location(cube: &RubiksCube, edge_1: &Color, edge_2: &Color) -> Vec<(usize, usize)> {
    /*
    This function finds the location for the edge piece
    */
    // first we locate all edges with color edge_1
    let location = find_edges_with_color(cube, edge_1);
    // then we find the color on the opposite side and if it matches with edge_2, we return that
    for i in 0..location.len() {
        // convert local into global
        let (x, y, z) = local_to_global(location[i].0, location[i].1);
        // convert back to local
        let mut local_positions = global_to_local(x, y, z);
        local_positions.retain(|&x| x != location[i]);
        
        // if the local_position matches the edge_2 color, we return these locations
        if cube.faces[local_positions[0].0][local_positions[0].1] == *edge_2 {
            return vec![location[i], local_positions[0]]
        }
        
    }
    
    // otherwise, we return (3, 9) and this will give an error later
    vec![(3, 9)]
    
}

// fn corner_piece_location(cube: &RubiksCube, target: &Color) -> HashMap<Vec<Color>, Vec<(usize, usize)>>{
fn corner_piece_location(cube: &RubiksCube, target: &Color, corner_1: &Color, corner_2: &Color) -> Vec<(usize, usize)>{
    /*
    This function finds the location of all corner pieces with the specified target color
    */
    // first find all the location with corner pieces with the target color
    let location = find_corner_with_color(cube, corner_1);
    
    // next we determine the color of the other 2 colors on the corner piece
    // to do this, we convert the local coordinate system into global, then back again into local
    for i in 0..location.len() {
        // convert local into global
        let (x, y, z) = local_to_global(location[i].0, location[i].1);
        // convert back to local
        let mut local_positions = global_to_local(x, y, z);
        local_positions.retain(|&x| x != location[i]);
        
        // if the local_position matches the edge_2 color, we return these locations
        if cube.faces[local_positions[0].0][local_positions[0].1] == *corner_2 && cube.faces[local_positions[1].0][local_positions[1].1] == *target {
            return vec![location[i], local_positions[0]]
        } else if cube.faces[local_positions[1].0][local_positions[1].1] == *corner_2 && cube.faces[local_positions[0].0][local_positions[0].1] == *target {
            return vec![location[i], local_positions[1]]
        }
    }
    vec![(3, 9)]
    
    
}