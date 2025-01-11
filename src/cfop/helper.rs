// This file contains helper functions for all files used in CFOP
use crate::rubiks::cube::RubiksCube;
use crate::rubiks::color::Color;
use std::{collections::HashSet, time::Duration};

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
        let mut move_notation = &move_type[0..1]; 

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

// for outputting data into excel
use umya_spreadsheet::*;

pub fn output_data(scramble: (&str, usize), 
                    cross_data: (Vec<String>, usize, Duration), 
                    f2l_data: (Vec<String>, usize, Duration), 
                    oll_data: (Vec<String>, usize, Duration), 
                    pll_data: (Vec<String>, usize, Duration),
                    total_data: (Vec<String>, usize, Duration)) -> Result<(), Box<dyn std::error::Error>> {

    /*
    This function outputs data into a excel file
        */
    // Open the Excel file
    let path = "src/cfop/analysis.xlsx";
    let mut workbook = reader::xlsx::read(std::path::Path::new(path)).unwrap();

    // Specify the sheet name to read
    // Step 2: Access a specific worksheet
    let sheet = workbook.get_sheet_by_name_mut("raw data").unwrap();

    // Iterate over rows and check for non-empty rows
    // for (index, row) in sheet.get_row_collection().enumerate() {
    for row_num in 1..=u32::MAX {
        let default = Cell::default();
        let first_cell = sheet.get_cell_by_column_and_row(1, row_num).unwrap_or(&default);
        // Get the first cell of the row (column A)
        // Check if the cell's value matches the target string
        if first_cell.get_value() == scramble.0 {
            return Ok(())
        }
        if !first_cell.get_value().is_empty() {
            continue
        }
        // If it doesn't exist, that means we add values to it
        else {
            // now we add the values into the excel sheet
            // Define the cells and their new values (row, column, value)
            let updates = vec![
                (row_num, 1, scramble.0.to_string()),   // Row _, Column 1
                (row_num, 2, scramble.1.to_string()),    // Row _, Column 2
                (row_num, 3, cross_data.0.join(" ")),// Row _, Column 3
                (row_num, 4, cross_data.1.to_string()),// Row _, Column 4
                (row_num, 5, cross_data.2.as_millis().to_string()),// Row _, Column 5
                (row_num, 6, f2l_data.0.join(" ")),// Row _, Column 6
                (row_num, 7, f2l_data.1.to_string()),// Row _, Column 7
                (row_num, 8, f2l_data.2.as_millis().to_string()),// Row _, Column 8
                (row_num, 9, oll_data.0.join(" ")),// Row _, Column 9
                (row_num, 10, oll_data.1.to_string()),// Row _, Column 10
                (row_num, 11, oll_data.2.as_millis().to_string()),// Row _, Column 11
                (row_num, 12, pll_data.0.join(" ")),// Row _, Column 12
                (row_num, 13, pll_data.1.to_string()),// Row _, Column 13
                (row_num, 14, pll_data.2.as_millis().to_string()),// Row _, Column 14
                (row_num, 15, total_data.0.join(" ")),// Row _, Column 15
                (row_num, 16, total_data.1.to_string()),// Row _, Column 16
                (row_num, 17, total_data.2.as_millis().to_string()),// Row _, Column 17
            ];
        
            // Loop through each update and apply it
            for (row, col, value) in updates {
                sheet.get_cell_by_column_and_row_mut(col, row).set_value(value);
            }
            
            // // Open the same file and overwrite the original content
            let _ = writer::xlsx::write(&workbook, std::path::Path::new(path));
            // finally return
            return Ok(())

        }
            
        

    }


    Ok(())

                    
}
