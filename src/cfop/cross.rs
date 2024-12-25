// File to solve the cross in CFOP
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;



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

    println!("{:?}", edges);
    
    // Step 4: Move the edges to the correct location
    


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
    This function determines the location of all side colors
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

    println!("{:?}", center);
    center
}

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
