// This file performs the whole cfop

use crate::rubiks::cube::RubiksCube;
use crate::cfop::cross::solve_cross;
use crate::cfop::f2l::solve_f2l;
use crate::cfop::oll::solve_oll;
use crate::cfop::pll::solve_pll;
use crate::helper::utils::*;

use std::time::{Duration, Instant};
use umya_spreadsheet::*;

pub fn cfop_solver(scramble: &str, mut cube: RubiksCube) -> Vec<String> {
    /*
    This function solves using the cfop 
    */
    // Determine the color of the bottom and top face
    let bottom = cube.faces[1][4];
    let top = cube.faces[0][4];
    
    // Step 1: Solve the cross
    println!("-------------cross-------------");
    let start_time = Instant::now();
    // let mut cube_cross = solve_cross(&mut cube, &target);
    let cross_moves = solve_cross(&mut cube, &bottom);
    let cross_elapsed_time = start_time.elapsed();
    println!("{}", cross_moves.join(" "));
    println!("Number of Moves: {}", cross_moves.len());
    println!("Elapsed time: {:?}", cross_elapsed_time);
    
    // Step 2: Solve the first 2 layers
    println!("-------------F2L-------------");
    let start_time = Instant::now();
    // let mut cube_f2l = solve_f2l(&mut cube_cross, &target);
    let f2l_moves = solve_f2l(&mut cube, &bottom);
    let f2l_elapsed_time = start_time.elapsed();
    println!("{}", f2l_moves.join(" "));
    println!("Number of Moves: {}", f2l_moves.len());
    println!("Elapsed time: {:?}", f2l_elapsed_time);
    
    // Step 3: Solve OLL
    println!("-------------OLL-------------");
    let start_time = Instant::now();
    let oll_moves = solve_oll(&mut cube, &top);
    let oll_elapsed_time = start_time.elapsed();
    println!("{}", oll_moves.join(" "));
    println!("Number of Moves: {}", oll_moves.len());
    println!("Elapsed time: {:?}", oll_elapsed_time);
    
    // Step 4: Solve PLL
    println!("-------------PLL-------------");
    let start_time = Instant::now();
    let pll_moves = solve_pll(&mut cube);
    let pll_elapsed_time = start_time.elapsed();
    println!("{}", pll_moves.join(" "));
    println!("Number of Moves: {}", pll_moves.len());
    println!("Elapsed time: {:?}", pll_elapsed_time);
    
    // Step 5: Total Moves
    println!("-------------Total-------------");
    let mut total_moves = Vec::new();
    total_moves.extend(cross_moves.clone());
    total_moves.extend(f2l_moves.clone());
    total_moves.extend(oll_moves.clone());
    total_moves.extend(pll_moves.clone());
    let total_moves_cleaned = cleanup_moves(total_moves);
    println!("{}", total_moves_cleaned.join(" "));
    println!("Number of Moves: {}", total_moves_cleaned.len());
    
    // Output data into excel file
    let _ = output_data((scramble, scramble.split(" ").collect::<Vec<&str>>().len()), 
    (cross_moves.clone(), cross_moves.len(), cross_elapsed_time),
    (f2l_moves.clone(), f2l_moves.len(), f2l_elapsed_time),
    (oll_moves.clone(), oll_moves.len(), oll_elapsed_time),
    (pll_moves.clone(), pll_moves.len(), pll_elapsed_time),
    (total_moves_cleaned.clone(), total_moves_cleaned.len(), cross_elapsed_time+f2l_elapsed_time+oll_elapsed_time+pll_elapsed_time));
    
    total_moves_cleaned
}

// for outputting data into excel
fn output_data(scramble: (&str, usize), 
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
    // let path = "src/cfop/path_analysis.xlsx";
    let mut workbook = reader::xlsx::read(std::path::Path::new(path)).unwrap();

    // Specify the sheet name to read
    // Step 2: Access a specific worksheet
    // let sheet = workbook.get_sheet_by_name_mut("raw data").unwrap();
    let sheet = workbook.get_sheet_by_name_mut("dijkstra").unwrap();

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
                (row_num, 5, cross_data.2.as_micros().to_string()),// Row _, Column 5
                (row_num, 7, f2l_data.0.join(" ")),// Row _, Column 7
                (row_num, 8, f2l_data.1.to_string()),// Row _, Column 8
                (row_num, 9, f2l_data.2.as_micros().to_string()),// Row _, Column 9
                (row_num, 11, oll_data.0.join(" ")),// Row _, Column 11
                (row_num, 12, oll_data.1.to_string()),// Row _, Column 12
                (row_num, 13, oll_data.2.as_micros().to_string()),// Row _, Column 13
                (row_num, 15, pll_data.0.join(" ")),// Row _, Column 15
                (row_num, 16, pll_data.1.to_string()),// Row _, Column 16
                (row_num, 17, pll_data.2.as_micros().to_string()),// Row _, Column 17
                (row_num, 19, total_data.0.join(" ")),// Row _, Column 19
                (row_num, 20, total_data.1.to_string()),// Row _, Column 20
                (row_num, 21, total_data.2.as_micros().to_string()),// Row _, Column 21
            ];
        
            // Loop through each update and apply it
            for (row, col, value) in updates {
                assert!(col >= 1, "Column number starts from 1.");
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