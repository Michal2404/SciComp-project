// This file contains helper functions for all files used in CFOP
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;
use std::collections::{HashMap, HashSet};

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

pub fn cleanup_moves(output_list: Vec<String>) -> Vec<String>{
    /*
    This function iterates through the moves and cleans up repeating and unnecessary moves
     */
    let mut simplified_output_list = Vec::new();
    let mut i = 0;

    while i < output_list.len() {
        let move_type = &output_list[i];
        let mut count = 0;
        let move_notation = &move_type[0..1]; 

        if move_type.ends_with("'") {
            count -= 1
        }
        else if move_type.ends_with("2") {
            count += 2
        }
        else {
            count += 1
        }

        // Count consecutive identical moves (ignore direction)
        while i + 1 < output_list.len() && output_list[i + 1][0..1] == move_type[0..1] {
            // we will assume clockwise is +ve and counterclockwise is -ve
            // for counterclockwise, subtract 1
            if output_list[i + 1].ends_with("'"){
                count -= 1
            }
            // for double moves, we add 2
            else if output_list[i + 1].ends_with("2"){
                count += 2
            } 
            // for clockwise, add 1
            else {
                count += 1
            }
            i += 1;
        }

        // Simplify based on the count modulo 4
        match i32::abs(count % 4) {
            1 => {
                if count < 0 { simplified_output_list.push(format!("{}'", move_notation)) } else { simplified_output_list.push(format!("{}", move_notation)) }
                
            }, // Single move
            2 => {
                simplified_output_list.push(format!("{}2", move_notation)); // Double move
                
            }
            3 => {
                // Everything becomes the opposite
                if count < 0 { simplified_output_list.push(format!("{}", move_notation)) } else { simplified_output_list.push(format!("{}'", move_notation)) }
            }
            _ => {} // Do nothing for a multiple of 4
        }

        i += 1;
    }

    simplified_output_list

}

pub fn local_to_global(a: usize, b: usize) -> (usize, usize, usize) {
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

pub fn global_to_local(x: usize, y: usize, z: usize) -> Vec<(usize, usize)>{
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
