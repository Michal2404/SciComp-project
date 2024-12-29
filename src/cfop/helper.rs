// This file contains helper functions for all files used in CFOP
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;

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