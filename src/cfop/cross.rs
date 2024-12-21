// File to solve the cross in CFOP
// use rubiks::cube::RubiksCube;
// use rubiks::color::Color;
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;
// use std::vec::Vec;

impl RubiksCube{

    pub fn solve_cross(&self) {
        /*
        This function solves the cross depending on what color we choose
         */
        // Step 1: Determine the color of the bottom face
        let target = self.faces[1][4];

        // Step 2: Locate all edges with the target color
        let edges = self.find_edges_with_color(&target);

        // Step 3: Move the edges to the correct location
        
        println!("{:?}", edges);


    }

    fn find_edges_with_color(&self, target: &Color) -> Vec<(usize, usize)> {
        /*
        This function finds the edges with the target colors
         */
        let mut edges: Vec<(usize, usize)> = Vec::new();
        for i in 0..6 as usize {
            for j in 0..9 as usize {
                // if self.faces[i][j] == *target && self.is_edge_position(j) {
                if self.faces[i][j] == *target && (j == 1 || j == 3 || j == 5 || j == 7) {
                    edges.push((i, j));
                }
            }
        }
        return edges
    }

    // fn is_edge_position(i: usize) -> bool {
    //     /*
    //     Determine if a position is an edge (not a corner or center)
    //     */
    //     return i == 1 || i == 3 || i == 5 || i == 7
    // }

}