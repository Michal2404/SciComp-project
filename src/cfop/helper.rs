// This file contains helper functions for all files used in CFOP
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;
use std::collections::HashSet;

pub fn find_edges_with_color(cube: &RubiksCube, target: &Color) -> Vec<(usize, usize)> {
    /*
    This function finds the edges with the target color
     */
    let mut edges: Vec<(usize, usize)> = Vec::new();
    // loop through all positions
    for i in 0..6 as usize {
        for j in 0..9 as usize {
            // if the edge has the target color, we return this
            if cube.faces[i][j] == *target && (j == 1 || j == 3 || j == 5 || j == 7) {
                edges.push((i, j));
            }
        }
    }
    return edges
}

pub fn find_corner_with_color(cube: &RubiksCube, target: &Color) -> Vec<(usize, usize)> {
    /*
    This function finds the corners with the target color
     */
    let mut corner: Vec<(usize, usize)> = Vec::new();
    // loop through all positions
    for i in 0..6 as usize {
        for j in 0..9 as usize {
            // if the edge has the target color, we return this
            if cube.faces[i][j] == *target && (j == 0 || j == 2 || j == 6 || j == 8) {
                corner.push((i, j));
            }
        }
    }
    return corner
}

pub fn parse_vec_color(content: &str) -> Vec<Color> {
    /*
    This functioon parses the string [Color, Color] into Vec<Color>
     */
    // Remove square brackets and whitespace, then split by `), (`
    let cleaned = content.trim().trim_matches(|c| c == '[' || c == ']');
    let colors = cleaned.split(", ");

    // Parse each pair into a tuple
    let mut result = Vec::new();
    for color in colors{
        match color {
            "W" => result.push(Color::W),
            "Y" => result.push(Color::Y),
            "G" => result.push(Color::G),
            "B" => result.push(Color::B),
            "R" => result.push(Color::R),
            "O" => result.push(Color::O),
            _ => ()
        } 
    }
    result
}

pub fn parse_vec_usize(content: &str) -> Vec<(usize,usize)> {
    /*
    This function parses the string [(usize, usize), (usize, usize)] into Vec<(usize, usize)>
     */
    // Remove square brackets and whitespace, then split by `), (`
    let cleaned = content.trim().trim_matches(|c| c == '[' || c == ']');
    let pairs = cleaned.split("), (");

    // Parse each pair into a tuple
    let mut result = Vec::new();
    for pair in pairs {
        let numbers: Vec<&str> = pair.trim_matches(|c| c == '(' || c == ')').split(',').collect();
        if numbers.len() == 2 {
            if let (Ok(x), Ok(y)) = (numbers[0].trim().parse::<usize>(), numbers[1].trim().parse::<usize>()) {
                result.push((x, y));
            }
        }
    }
    result
}

pub fn contains_element(key_set: HashSet<(usize, usize)>, position: Vec<(usize, usize)>) -> bool {
    /*
    This function determines if key_set and position contains the same elements regardless of order
     */
    position.iter().all(|item| key_set.contains(item))
}