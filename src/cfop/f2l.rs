// File to solve the first two layers in CFOP
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;
use crate::cfop::helper::*;
use std::collections::HashMap;



pub fn solve_f2l(cube: &RubiksCube, target: &Color) {
    /*
    This function solves the f2l
     */
    // let a = 0;
    // let b = 3;

    // let (x, y, z) = local_to_global(a, b);
    // let pos = global_to_local(x, y, z);

    // println!("a={}, b={}", a, b);
    // println!("x={}, y={}, z={}", x, y, z);
    // for i in 0..pos.len() {
    //     println!("i={}, j={}", pos[i].0, pos[i].1);
    // }

    // first we determine the color of the corner and its location in global coordinate system
    let corner_locations = corner_piece_location(cube, target);
    println!("{:?}", corner_locations);

    // next find the location of the edges corresponding to the colors
    let mut edge_locations: HashMap<Vec<Color>, Vec<(usize, usize)>> = HashMap::new();
    for (key, val) in corner_locations.into_iter() {
        // find the edge_piece_location corresponding to the 2 colors
        let edges: Vec<(usize, usize)> = edge_piece_location(cube, &key[0], &key[1]);
        edge_locations.insert(key, edges);
    }
    println!("{:?}", edge_locations);

    // now we need to know where the pieces should end up in



    
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
    return vec![(3, 9)]

}
    
fn corner_piece_location(cube: &RubiksCube, target: &Color) -> HashMap<Vec<Color>, Vec<(usize, usize)>>{
    /*
    This function finds the location of all corner pieces with the specified target color
        */
    // first find all the location with corner pieces with the target color
    let corner = find_corner_with_color(cube, target);

    // next we determine the color of the other 2 colors on the corner piece
    // to do this, we convert the local coordinate system into global, then back again into local
    let mut location: HashMap<Vec<Color>, Vec<(usize, usize)>> = HashMap::new();
    // let mut location: HashMap<(usize, usize, usize), (Color, Color)> = HashMap::new();
    for i in 0..corner.len() {
        // convert local into global
        let (x, y, z) = local_to_global(corner[i].0, corner[i].1);
        // convert back to local
        let mut local_positions = global_to_local(x, y, z);
        local_positions.retain(|&x| x != corner[i]);

        // determine the color of the other 2 faces
        // location.insert((x,y,z), (cube.faces[local_positions[0].0][local_positions[0].1], cube.faces[local_positions[1].0][local_positions[1].1]));
        location.insert(vec![cube.faces[local_positions[0].0][local_positions[0].1], cube.faces[local_positions[1].0][local_positions[1].1]], local_positions);
    }
    return location


}
fn local_to_global(a: usize, b: usize) -> (usize, usize, usize) {
    /*
    This function converts the local coordinate system (2d array), into global coordinate system (3d array)
    a = face number
    b = location on face
    output = center is (1,1,1)
     */

    // look at the face number
    let mut x: usize = 3;
    let mut y: usize = 3;
    let mut z: usize = 3;
    match a {
        0 => z = 2,
        1 => z = 0,
        2 => y = 0,
        3 => y = 2,
        4 => x = 2,
        5 => x = 0,
        _ => panic!("a should be between 0 and 5"),
    }
    
    // next look at location on the face
    let (i, j) = match b {
        0 => if x == 2 || y == 0 || z == 2 {(0, 2)} else if z == 0 {(0, 0)} else {(2, 2)},
        1 => (1, 2),
        2 => if x == 2 || y == 0 || z == 2 {(2, 2)} else if z == 0 {(2, 0)} else {(0, 2)},
        3 => if x == 2 || y == 0 || z == 2 {(0, 1)} else if z == 0 {(0, 1)} else {(2, 1)}
        4 => (1, 1),
        5 => if x == 2 || y == 0 || z == 2 {(2, 1)} else if z == 0 {(2, 1)} else {(0, 1)},
        6 => if x == 2 || y == 0 || z == 2 {(0, 0)} else if z == 0 {(0, 2)} else {(2, 0)},
        7 => (1, 0),
        8 => if x == 2 || y == 0 || z == 2 {(2, 0)} else if z == 0 {(2, 2)} else {(0, 0)},
        _ => panic!("b should be between 0 and 8"),
    };

    // lastly we will change x, y, or z depending on which value is 3
    if x != 3 {
        y = i;
        z = j
    } else if y != 3 {
        x = i;
        z = j;
    } else if z != 3 {
        x = i;
        y = j;
    } else { panic!("all x y z are 3") };

    (x, y, z)


}

fn global_to_local(x: usize, y: usize, z: usize) -> Vec<(usize, usize)>{
    /*
    This function converts global coordinate system into local coordinate system
     */
    // specify the local coordinate system
    let a: Vec<usize> = vec![0, 1, 2, 0, 1, 2, 0, 1, 2];
    let b: Vec<usize> = vec![2, 2, 2, 1, 1, 1, 0, 0, 0];

    let mut local_position: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..a.len() {
        local_position.insert((a[i], b[i]), i);
    }

    let coordinate = vec![x, y, z];
    let mut local_coordinate: Vec<(usize, usize)> = Vec::new();

    for (index, cut) in coordinate.iter().enumerate() {
        // we only find the location if cut doesn't equal 1 
        if *cut != 1 {
            // we first clone coordinate and rmeove the index
            let mut temp = coordinate.clone();
            temp.remove(index);

            // next we transform the global coordinate system to local coordinate system using a and b directions
            // for x and y, we need to change the coordinate, for a
            if (index == 0 && *cut == 0) || (index == 1 && *cut == 2) {
                match temp[0] {
                    0 => temp[0] = 2,
                    2 => temp[0] = 0,
                    _ => (),
                }
            }
            // for z, we need to change the coordinate, for b
            else if index == 2 && *cut == 0 {
                match temp[1] {
                    0 => temp[1] = 2,
                    2 => temp[1] = 0,
                    _ => (),
                }
            }
            // otherwise, we don't need to change anything
            else {}
            
            // get the index of local coordinate
            let position = local_position.get(&(temp[0], temp[1])).unwrap();
            
            // lastly face will depend on the cut
            let face = match (index, cut) {
                (0, 0) => 5,
                (0, 2) => 4,
                (1, 0) => 2,
                (1, 2) => 3,
                (2, 0) => 1,
                (2, 2) => 0,
                _ => panic!("something is wrong here")
            };
            local_coordinate.push((face, *position));
        }

    }
    local_coordinate
}
