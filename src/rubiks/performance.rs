use crate::rubiks::solver::solve;

use super::bfs::{bfs_solver, ida_star_solver};
use super::cubie::{generate_scramble, CubieCube};
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

/// Measure the performance of IDA* start algorithm depending of different scramble lengths.
pub fn measure_ida(n: usize) -> Result<(), Box<dyn Error>> {
    let max_depth = 20;
    let mut results = Vec::new();

    for scramble_length in 1..=8 {
        println!("it: {}", scramble_length);
        // Generate a scramble of the given length

        let mut valid_solution = None;
        let mut attempts = 0;
        let mut start_ida = Instant::now(); // Start the timer here
        let mut time_ida = start_ida.elapsed(); // Initialize `time_ida` for later use

        // Keep solving until a solution with the correct length is found
        while valid_solution.is_none() {
            let scramble = generate_scramble(scramble_length);
            let cube = CubieCube::from_scramble(&scramble);
            start_ida = Instant::now();
            let moves_ida = ida_star_solver(&cube, max_depth);
            attempts += 1;

            if let Some(solution) = moves_ida {
                if solution.len() == scramble_length {
                    valid_solution = Some(solution);
                    time_ida = start_ida.elapsed(); // Capture the elapsed time when a valid solution is found
                } else {
                    println!(
                        "Attempt {}: Found solution with length {} for scramble length {}, retrying...",
                        attempts,
                        solution.len(),
                        scramble_length
                    );
                }
            } else {
                println!(
                    "Attempt {}: No solution found for scramble length {}, retrying...",
                    attempts, scramble_length
                );
            }
        }

        // Collect the results for the first valid solution
        if let Some(solution) = valid_solution {
            results.push((scramble_length, solution.len(), time_ida.as_secs_f64()));
        }
    }

    // Write results to a CSV file
    let format_string =
        format!("csv_files/ida_performance/ida_performance_new_heuristics{n}_sclen8.csv");
    let filename = format_string.as_str();
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(["Scramble Length", "IDA* Moves", "IDA* Time (s)"])?;

    // Write rows
    for (length, ida_moves, ida_time) in &results {
        wtr.write_record(&[
            length.to_string(),
            ida_moves.to_string(),
            ida_time.to_string(),
        ])?;
    }

    wtr.flush()?;
    println!("Results written to {}", filename);

    // Print results as a table
    println!("+----------------+------------+-----------------+");
    println!("| Scramble Length | IDA* Moves | IDA* Time (s)  |");
    println!("+----------------+------------+-----------------+");
    for (length, ida_moves, ida_time) in results {
        println!(
            "| {:<15} | {:<10} | {:<14.6} |",
            length, ida_moves, ida_time
        );
    }
    println!("+----------------+------------+----------------+");

    Ok(())
}

/// Measure the performance of BFS algorithm depending on scramble length.
pub fn measure_bfs(n: usize) -> Result<(), Box<dyn Error>> {
    let max_depth = 20;
    let mut results = Vec::new();

    for scramble_length in 1..=6 {
        println!("it: {}", scramble_length);
        // Generate a scramble of the given length

        let mut valid_solution = None;
        let mut attempts = 0;
        let mut start_bfs = Instant::now(); // Start the timer here
        let mut time_bfs = start_bfs.elapsed(); // Initialize `time_ida` for later use

        // Keep solving until a solution with the correct length is found
        while valid_solution.is_none() {
            let scramble = generate_scramble(scramble_length);
            let cube = CubieCube::from_scramble(&scramble);
            start_bfs = Instant::now();
            let moves_bfs = bfs_solver(&cube, max_depth);
            attempts += 1;

            if let Some(solution) = moves_bfs {
                if solution.len() == scramble_length {
                    valid_solution = Some(solution);
                    time_bfs = start_bfs.elapsed(); // Capture the elapsed time when a valid solution is found
                } else {
                    println!(
                        "Attempt {}: Found solution with length {} for scramble length {}, retrying...",
                        attempts,
                        solution.len(),
                        scramble_length
                    );
                }
            } else {
                println!(
                    "Attempt {}: No solution found for scramble length {}, retrying...",
                    attempts, scramble_length
                );
            }
        }

        // Collect the results for the first valid solution
        if let Some(solution) = valid_solution {
            results.push((scramble_length, solution.len(), time_bfs.as_secs_f64()));
        }
    }

    // Write results to a CSV file
    let format_string = format!("csv_files/bfs_performance/bfs_performance_{n}.csv");
    let filename = format_string.as_str();
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(["Scramble Length", "BFS Moves", "BFS Time (s)"])?;

    // Write rows
    for (length, bfs_moves, bfs_time) in &results {
        wtr.write_record(&[
            length.to_string(),
            bfs_moves.to_string(),
            bfs_time.to_string(),
        ])?;
    }

    wtr.flush()?;
    println!("Results written to {}", filename);

    // Print results as a table
    println!("+----------------+------------+-----------------+");
    println!("| Scramble Length | BFS Moves | BFS Time (s)  |");
    println!("+----------------+------------+-----------------+");
    for (length, bfs_moves, bfs_time) in results {
        println!(
            "| {:<15} | {:<10} | {:<14.6} |",
            length, bfs_moves, bfs_time
        );
    }
    println!("+----------------+------------+----------------+");

    Ok(())
}

// Measure the performance of the two phase solver
pub fn measure_two_phase() -> Result<(), Box<dyn Error>> {
    let mut results = Vec::new();
    let init_scramble = "R U L F D";
    let _solution = solve(&init_scramble, 20, 2.0, true, false, Some(10));
    for i in 0..10000 {
        println!("it: {}", i);
        for scramble_length in 1..=30 {
            // Generate a scramble of the given length

            let scramble = generate_scramble(scramble_length);
            let start_time = Instant::now();
            let solution = solve(&scramble, 20, 2.0, true, false, Some(8));
            let end_time = start_time.elapsed();
            let trimmed_solution = solution
                .rsplit_once('(')
                .map_or(solution.clone(), |(before, _)| before.trim().to_string());
            let solution_string = trimmed_solution.trim().to_string();
            let solution_length = solution_string.split_whitespace().count();
            results.push((scramble_length, solution_length, end_time.as_secs_f64()));
        }
    }
    // Write results to a CSV file
    let format_string = format!("csv_files/real_experiments/performance_no_ida_it_10000.csv");
    let filename = format_string.as_str();
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(["Scramble Length", "Solution Moves", "Solution Time (s)"])?;

    // Write rows
    for (length, moves, time) in &results {
        wtr.write_record(&[length.to_string(), moves.to_string(), time.to_string()])?;
    }

    wtr.flush()?;
    println!("Results written to {}", filename);

    // Print results as a table
    println!("+-----------------+---------+-----------+");
    println!("| Scramble Length |  Moves  | Time (s)  |");
    println!("+-----------------+---------+-----------+");
    for (length, moves, time) in results {
        println!("| {:<15} | {:<10} | {:<14.6} |", length, moves, time);
    }
    println!("+----------------+------------+----------------+");

    Ok(())
}

pub fn measure_two_phase_ida() -> Result<(), Box<dyn Error>> {
    let mut results = Vec::new();
    let init_scramble = "R U L F D";
    let _solution = solve(&init_scramble, 20, 2.0, true, false, Some(10));
    for i in 0..10000 {
        println!("it: {}", i);
        for scramble_length in 1..=30 {
            // Generate a scramble of the given length

            let scramble = generate_scramble(scramble_length);
            let start_time = Instant::now();
            let solution = solve(&scramble, 20, 2.0, true, true, Some(8));
            let end_time = start_time.elapsed();
            let trimmed_solution = solution
                .rsplit_once('(')
                .map_or(solution.clone(), |(before, _)| before.trim().to_string());
            let solution_string = trimmed_solution.trim().to_string();
            let solution_length = solution_string.split_whitespace().count();
            results.push((scramble_length, solution_length, end_time.as_secs_f64()));
        }
    }
    // Write results to a CSV file
    let format_string = format!("csv_files/real_experiments/performance_ida_it_10000.csv");
    let filename = format_string.as_str();
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(["Scramble Length", "Solution Moves", "Solution Time (s)"])?;

    // Write rows
    for (length, moves, time) in &results {
        wtr.write_record(&[length.to_string(), moves.to_string(), time.to_string()])?;
    }

    wtr.flush()?;
    println!("Results written to {}", filename);

    // Print results as a table
    println!("+-----------------+---------+-----------+");
    println!("| Scramble Length |  Moves  | Time (s)  |");
    println!("+-----------------+---------+-----------+");
    for (length, moves, time) in results {
        println!("| {:<15} | {:<10} | {:<14.6} |", length, moves, time);
    }
    println!("+----------------+------------+----------------+");

    Ok(())
}
