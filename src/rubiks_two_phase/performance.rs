use crate::rubiks_two_phase::solver::solve;

use super::bfs::{ida_star_solver, iddfs_solver};
use super::cubie::{generate_scramble, CubieCube};
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

/// Measure the performance of IDA* start algorithm depending of different scramble lengths.
pub fn measure_ida() -> Result<(), Box<dyn Error>> {
    let max_depth = 20;
    let mut results = Vec::new();
    // Load tables
    let init_scramble = "R U L F D";
    let _solution = solve(init_scramble, 20, 2.0, true, false, Some(10));
    for i in 0..100 {
        println!("it: {}", i);
        for scramble_length in 1..=12 {
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
                results.push((solution.len(), time_ida.as_secs_f64()));
            }
        }
    }

    // Write results to a CSV file
    let filename = "csv_files/real_experiments/IDA_time_performance.csv";
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(["IDA* Moves", "IDA* Time (s)"])?;

    // Write rows
    for (ida_moves, ida_time) in &results {
        wtr.write_record(&[ida_moves.to_string(), ida_time.to_string()])?;
    }

    wtr.flush()?;
    println!("Results written to {}", filename);

    // Print results as a table
    println!("+------------+----------------+");
    println!("| IDA* Moves | IDA* Time (s)  |");
    println!("+------------+----------------+");
    for (ida_moves, ida_time) in results {
        println!("| {:<10} | {:<14.6} |", ida_moves, ida_time);
    }
    println!("+----------------+------------+");

    Ok(())
}

/// Measure the performance of two phase solver with IDA* star depending on ida search depth
/// We are measuring the time performance of the solver with active IDA* option depending on
/// the IDA* search depth. We want to find a perfect value for the depth s.t. the solving time
/// is in 99% of cases lower than 200ms.
pub fn measure_ida_depth_performance() -> Result<(), Box<dyn Error>> {
    let mut results = Vec::new();
    // Load tables
    let init_scramble = "R U L F D";
    let _solution = solve(init_scramble, 20, 2.0, true, false, Some(10));
    // Start experiment
    for i in 0..100 {
        println!("it: {}", i);
        for max_depth in 1..=8 {
            // Generate a random scramble of length 30
            let scramble = generate_scramble(30);

            let start_ida = Instant::now();
            let _solution = solve(&scramble, 20, 2.0, true, true, Some(max_depth));
            let time_ida = start_ida.elapsed();

            results.push((max_depth, time_ida.as_secs_f64()));
        }
    }
    // Write results to a CSV file
    let filename = "csv_files/real_experiments/performance_ida_depth_vs_time_8.csv";
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(["ida_depth", "Time (s)"])?;

    // Write rows
    for (max_depth, time_ida) in &results {
        wtr.write_record(&[max_depth.to_string(), time_ida.to_string()])?;
    }

    wtr.flush()?;
    println!("Results written to {}", filename);

    // Print results as a table
    println!("+------------+----------------+");
    println!("| IDA* Depth | IDA* Time (s)  |");
    println!("+------------+----------------+");
    for (max_depth, time_ida) in results {
        println!("| {:<15} | {:<14.6} |", max_depth, time_ida);
    }
    println!("+----------------+------------+----------------+");

    Ok(())
}

/// Measure the performance of BFS algorithm depending on scramble length.
pub fn measure_bfs() -> Result<(), Box<dyn Error>> {
    let mut results = Vec::new();
    // Load tables
    let init_scramble = "R U L F D";
    let _solution = solve(init_scramble, 20, 2.0, true, false, Some(10));

    for i in 0..100 {
        println!("it: {}", i);
        for scramble_length in 1..=8 {
            println!("scramble len: {}", scramble_length);
            // Generate a scramble of the given length

            let mut valid_solution = None;
            let mut attempts = 0;
            let start_bfs = Instant::now(); // Start the timer here
            let mut time_iddfs = start_bfs.elapsed(); // Initialize `time_ida` for later use

            // Keep solving until a solution with the correct length is found
            while valid_solution.is_none() {
                let scramble = generate_scramble(scramble_length);
                let cube = CubieCube::from_scramble(&scramble);
                let start = Instant::now();

                // Use iterative deepening DFS up to scramble_length (or another max depth)
                if let Some(solution) = iddfs_solver(&cube, scramble_length) {
                    if solution.len() == scramble_length {
                        valid_solution = Some(solution);
                        time_iddfs = start.elapsed();
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
                attempts += 1;
            }

            // Collect the results for the first valid solution
            if let Some(solution) = valid_solution {
                results.push((solution.len(), time_iddfs.as_secs_f64()));
            }
        }
    }

    // Write results to a CSV file
    let filename = "csv_files/real_experiments/BFS_time_performance.csv";
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(["BFS Moves", "BFS Time (s)"])?;

    // Write rows
    for (bfs_moves, bfs_time) in &results {
        wtr.write_record(&[bfs_moves.to_string(), bfs_time.to_string()])?;
    }

    wtr.flush()?;
    println!("Results written to {}", filename);

    // Print results as a table
    println!("+-----------+---------------+");
    println!("| BFS Moves | BFS Time (s)  |");
    println!("+-----------+---------------+");
    for (bfs_moves, bfs_time) in results {
        println!("| {:<10} | {:<14.6} |", bfs_moves, bfs_time);
    }
    println!("+----------------+------------+----------------+");

    Ok(())
}

// Measure the performance of the two phase solver
pub fn measure_two_phase() -> Result<(), Box<dyn Error>> {
    let mut results = Vec::new();
    let init_scramble = "R U L F D";
    let _solution = solve(init_scramble, 20, 2.0, true, false, Some(10));
    for i in 0..100000 {
        println!("it: {}", i);
        for scramble_length in 1..=20 {
            // Generate a scramble of the given length
            let scramble = generate_scramble(scramble_length);
            let start_time = Instant::now();
            let solution = solve(&scramble, 20, 2.0, true, false, Some(8));
            let end_time = start_time.elapsed();

            let solution_string = solution.join(" ").trim().to_string();
            let solution_length = solution_string.split_whitespace().count();
            results.push((scramble_length, solution_length, end_time.as_secs_f64()));
        }
    }
    // Write results to a CSV file
    let filename = "csv_files/real_experiments/Performance_2phase_02_02_HQ.csv";
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
    let _solution = solve(init_scramble, 20, 2.0, true, false, Some(10));
    for i in 0..10000 {
        println!("it: {}", i);
        for scramble_length in 1..=20 {
            // Generate a scramble of the given length

            let scramble = generate_scramble(scramble_length);
            let start_time = Instant::now();
            let solution = solve(&scramble, 20, 2.0, true, true, Some(6));
            let end_time = start_time.elapsed();

            let solution_string = solution.join(" ").trim().to_string();
            let solution_length = solution_string.split_whitespace().count();
            results.push((scramble_length, solution_length, end_time.as_secs_f64()));
        }
    }
    // Write results to a CSV file
    let filename = "csv_files/real_experiments/Performance_2phaseida_02_02_depth_06.csv";
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

pub fn two_phase_len_performance() -> Result<(), Box<dyn Error>> {
    // The results will hold two columns. One colums is the time, second column is the solution length
    let mut results = Vec::new();
    // Load the tables
    let init_scramble = "R U L F D";
    let _solution = solve(init_scramble, 20, 2.0, true, false, Some(10));
    for i in 0..100000 {
        println!("it: {}", i);
        // Generate random scramble with length 30, should be random enough
        let scramble = generate_scramble(30);
        let start_time = Instant::now();
        let solution = solve(&scramble, 20, 2.0, true, false, Some(8));
        let end_time = start_time.elapsed();

        let solution_string = solution.join("").trim().to_string();
        let solution_length = solution_string.split_whitespace().count();
        results.push((solution_length, end_time.as_secs_f64()));
    }
    // Write results to a CSV file
    let filename = "csv_files/real_experiments/two_phase_times_and_solution_lengths.csv";
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(["Length", "Time (ms)"])?;

    // Write rows
    for (length, time) in &results {
        wtr.write_record(&[length.to_string(), time.to_string()])?;
    }

    wtr.flush()?;
    println!("Results written to {}", filename);

    Ok(())
}
