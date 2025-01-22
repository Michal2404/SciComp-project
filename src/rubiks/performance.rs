use super::bfs::{bfs_solver, ida_star_solver};
use super::cubie::{generate_scramlbe, CubieCube};
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

/// Compare performance of IDA* and BFS algorithms for various scramble lengths and save
/// the result as .csv table.
pub fn compare_algorithms(n: usize) -> Result<(), Box<dyn Error>> {
    let max_depth = 20;
    let mut results = Vec::new();

    // Test scramble lengths from 1 to 6
    for scramble_length in 1..=6 {
        // Generate a scramble of the given length
        println!("Solving scramble len = {}", scramble_length);
        let scramble = generate_scramlbe(scramble_length);
        let cube = CubieCube::from_scramble(&scramble);

        // Measure BFS performance
        let start_bfs = Instant::now();
        let moves_bfs = bfs_solver(&cube, max_depth).unwrap();
        let time_bfs = start_bfs.elapsed();
        println!("BFS depth {}, time: {:?}", scramble_length, time_bfs);

        // Measure IDA* performance
        let start_ida = Instant::now();
        let moves_ida = ida_star_solver(&cube, max_depth).unwrap();
        let time_ida = start_ida.elapsed();
        println!("IDA* depth {}, time: {:?}", scramble_length, time_ida);

        // Collect results
        results.push((
            scramble_length,
            moves_bfs.len(),
            time_bfs.as_secs_f64(),
            moves_ida.len(),
            time_ida.as_secs_f64(),
        ));
    }

    // Write results to a CSV file
    let format_string = format!("csv_files/algorithm_comparison{n}.csv");
    let filename = format_string.as_str();
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(&[
        "Scramble Length",
        "BFS Moves",
        "BFS Time (s)",
        "IDA* Moves",
        "IDA* Time (s)",
    ])?;

    // Write rows
    for (length, bfs_moves, bfs_time, ida_moves, ida_time) in &results {
        wtr.write_record(&[
            length.to_string(),
            bfs_moves.to_string(),
            bfs_time.to_string(),
            ida_moves.to_string(),
            ida_time.to_string(),
        ])?;
    }

    wtr.flush()?;
    println!("Results written to {}", filename);

    // Print results as a table
    println!("+----------------+------------+----------------+------------+----------------+");
    println!("| Scramble Length | BFS Moves  | BFS Time (s)   | IDA* Moves | IDA* Time (s)  |");
    println!("+----------------+------------+----------------+------------+----------------+");
    for (length, bfs_moves, bfs_time, ida_moves, ida_time) in results {
        println!(
            "| {:<15} | {:<10} | {:<14.6} | {:<10} | {:<14.6} |",
            length, bfs_moves, bfs_time, ida_moves, ida_time
        );
    }
    println!("+----------------+------------+----------------+------------+----------------+");

    Ok(())
}

/// Measure the performance of IDA* start algorithm depending of different scramble lengths.
pub fn measure_ida(n: usize) -> Result<(), Box<dyn Error>> {
    let max_depth = 20;
    let mut results = Vec::new();

    // Test scramble lengths from 1 to 7
    for scramble_length in 1..=8 {
        println!("it: {}", scramble_length);
        // Generate a scramble of the given length
        let scramble = generate_scramlbe(scramble_length);
        let cube = CubieCube::from_scramble(&scramble);

        // Measure IDA* performance
        let start_ida = Instant::now();
        let moves_ida = ida_star_solver(&cube, max_depth).unwrap();
        let time_ida = start_ida.elapsed();

        // Collect results
        results.push((scramble_length, moves_ida.len(), time_ida.as_secs_f64()));
    }

    // Write results to a CSV file
    let format_string = format!("csv_files/ida_performance_{n}.csv");
    let filename = format_string.as_str();
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(&["Scramble Length", "IDA* Moves", "IDA* Time (s)"])?;

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

    // Test scramble lengths from 1 to 7
    for scramble_length in 1..=6 {
        println!("it: {}", scramble_length);
        // Generate a scramble of the given length
        let scramble = generate_scramlbe(scramble_length);
        let cube = CubieCube::from_scramble(&scramble);

        // Measure BFS performance
        let start_bfs = Instant::now();
        let moves_bfs = bfs_solver(&cube, max_depth).unwrap();
        let time_bfs = start_bfs.elapsed();

        // Collect results
        results.push((scramble_length, moves_bfs.len(), time_bfs.as_secs_f64()));
    }

    // Write results to a CSV file
    let format_string = format!("csv_files/bfs_performance/bfs_performance_{n}.csv");
    let filename = format_string.as_str();
    let mut wtr = Writer::from_writer(File::create(filename)?);

    // Write header
    wtr.write_record(&["Scramble Length", "BFS Moves", "BFS Time (s)"])?;

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
