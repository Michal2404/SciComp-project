use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::time::Instant;
use two_phase::gui::app::CubeVisualizerWithMoves;
use two_phase::rubiks::bfs::{bfs_solver, ida_star_solver};
use two_phase::rubiks::cubie::{generate_scramlbe, generate_states, CubieCube};

fn compare_algorithms() -> Result<(), Box<dyn Error>> {
    let max_depth = 20;
    let mut results = Vec::new();

    // Test scramble lengths from 1 to 7
    for scramble_length in 1..=5 {
        // Generate a scramble of the given length
        let scramble = generate_scramlbe(scramble_length);
        let cube = CubieCube::from_scramble(&scramble);

        // Measure BFS performance
        let start_bfs = Instant::now();
        let moves_bfs = bfs_solver(&cube, max_depth).unwrap();
        let time_bfs = start_bfs.elapsed();

        // Measure IDA* performance
        let start_ida = Instant::now();
        let moves_ida = ida_star_solver(&cube, max_depth).unwrap();
        let time_ida = start_ida.elapsed();

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
    let filename = "csv_files/algorithm_comparison.csv";
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

fn measure_ida(n: usize) -> Result<(), Box<dyn Error>> {
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

fn main() {
    // Create scrambled Cube
    /*
    let scramble = "R2 L2";
    let scrambled = cubie::CubieCube::from_scramble(&scramble);
    println!("Scrambled cube in Cubie notation:");
    println!("{:?}\n", scrambled);

    // Make FaceCube from CubieCube
    let face_scrambled = scrambled.to_facelet_cube().to_2dstring();
    println!("Scrambled cube in Facelet notation:");
    println!("{}\n", face_scrambled);

    // Inverse the Cube
    //let mut inverse = cubie::CubieCube::new(None, None, None, None);
    //scrambled.inv_cubie_cube(&mut inverse);
    //println!("Inverse of scrambled cube in Facelet notation:");
    //println!("{}\n", inverse.to_facelet_cube().to_2dstring());

    // Corner parity of scrambled cube
    //let corner_parity = scrambled.corner_parity();
    //println!("Corner parity of scramlbed cube:");
    //println!("{}\n", corner_parity);

    // Edge parity of scrambled cube
    //let edge_parity = scrambled.edge_parity();
    //println!("Edge parity of scramlbed cube:");
    //println!("{}\n", edge_parity);

    // Corner orientation coordinate of the scrambled cube phase 1
    let corner_orientation_coord = scrambled.get_twist();
    println!("Corner orientation coordinate of scrambled cube phase 1(0<=twist<2187):");
    println!("{}\n", corner_orientation_coord);

    // Edge orientation coordinate of the scrambled cube phase 1
    let edge_orientation_coord = scrambled.get_flip();
    println!("Edge orientation coordinate of scrambled cube phase 1 (0<=flip<2047):");
    println!("{}\n", edge_orientation_coord);

    // UD-Slice coordinate of the scrambled cube in phase 1
    let ud_slice_phase_1 = scrambled.get_slice();
    println!("Phase 1 UD-Slice coordinate (0<=x<=494)");
    println!("{}\n", ud_slice_phase_1);

    let cube_phase2 = CubieCube::from_scramble("U R2 L2 D' F2 U2 B2 R2 D");

    // Corner permutation coordinate of the scrambled cube phase 2
    let corner_permuration_coord = cube_phase2.get_corners();
    println!("Corner permutation coordinate of scrambled cube phase 2 (0<=x<=40319)");
    println!("{}\n", corner_permuration_coord);

    // Edge permutation coordinate of the scrambled cube phase 2
    let edge_permutation_coord = cube_phase2.get_ud_edges();
    println!("Edge permutation coordinate of scrambled cube phase 2 (0<=x<=40319)");
    println!("{}\n", edge_permutation_coord);

    // UD-Slice coordinate of the cube in phase 2
    let ud_slice_phase_2 = cube_phase2.get_slice_sorted();
    println!("Ud slice phase 2");
    println!("{}\n", ud_slice_phase_2);

    // Set corner twist
    //let mut twist: u16 = 10;
    //scrambled.set_twist(twist);

    //let mut parity_cube = cubie::CubieCube::new(None, None, None, None);
    //let mut twist: u16 = 27;
    //parity_cube.set_twist(twist);

    let coord_cube = CoordCube::from_cubie_cube(&scrambled);
    println!(
        "Cube in Coordiante representation: {}",
        coord_cube.to_string()
    );

    let cube_phase2_coord = CoordCube::from_cubie_cube(&cube_phase2);
    println!(
        "Cube phase 2 in Coordinate representation: {}",
        cube_phase2_coord.to_string()
    );

    // Testing getting the depth
    let depth = coord_cube.get_depth_phase1();
    println!("depth to the subgroup H: {}", depth);

    let depth2 = CoordCube::get_depth_phase2(scrambled.get_corners(), scrambled.get_ud_edges());
    println!("lower bound to depth phase 2 to solved cube: {}", depth2);



    // Visualize the scramble
    let app = CubeVisualizer::new(scrambled.to_facelet_cube());
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Cube Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    );
    */

    // Define as string
    /*
    let cubestring = "DUUBULDBFRBFRRULLLBRDFFFBLURDBFDFDRFRULBLUFDURRBLBDUDL";
    let mut fc = FaceCube::new();
    fc.from_string(&cubestring);
    let cubiecube = fc.to_cubie_cube();
    */
    // Define as scramble

    // Testing coords
    //println!("twist coord: {:?}", cubiecube.get_twist());
    //println!("flip coord: {:?}", cubiecube.get_flip());
    //println!("slice coord: {:?}", cubiecube.get_slice());
    //println!("slice sorted coord: {:?}", cubiecube.get_slice_sorted());
    //println!("u edges coord: {:?}", cubiecube.get_u_edges());
    //println!("d edges coord: {:?}", cubiecube.get_d_edges());
    //println!("corners coord: {:?}", cubiecube.get_corners());
    //println!("ud edges coord: {:?}", cubiecube.get_ud_edges());

    /*
    let cubescramble = generate_scramlbe(100);
    let cubescramble = cubescramble.as_str();
    let cubiecube = CubieCube::from_scramble(&cubescramble);

    let solution = sv::solve(cubescramble, 20, 2.0, true);
    println!("scramble:\n{}", cubescramble);
    println!("solution:\n{}", solution);

    let trimmed_solution = solution
        .rsplit_once('(')
        .map_or(solution.clone(), |(before, _)| before.trim().to_string());
    let states = generate_states(cubiecube.clone(), &trimmed_solution);

    */

    // Test performance BFS vs IDA*

    //if let Err(e) = compare_algorithms() {
    //    eprintln!("Error: {}", e);
    //}

    // Measure IDA*
    for n in 0..20 {
        if let Err(e) = measure_ida(n) {
            eprintln!("Error: {}", e);
        }
    }

    let cubiecube = CubieCube::new(None, None, None, None);
    let states = generate_states(cubiecube, "");
    let app = CubeVisualizerWithMoves::new(cubiecube.to_facelet_cube(), states, "".to_string());
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Cube Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    );
    // Testing the tables

    // phase2_edgemerge     -> DONE
    // move_twist           -> DONE
    // move_flip            -> DONE
    // slice_sorted_move    -> DONE
    // move_u_edges         -> DONE
    // move_d_edges         -> DONE
    // move_ud_edges        -> DONE
    // move_corners         -> DONE
    // phase1_prun          -> DONE (Very big numbers. Why?)
    // phase2_prun          -> DONE (Also very large numbers)
    // phase2_cornsliceprun -> DONE
    // conj_twist           -> DONE
    // conj_ud_edges        -> DONE
    // fs_classidx          -> DONE
    // fs_sym               -> DONE
    // fs_rep               -> DONE
    // co_classidx          -> DONE
    // co_sym               -> DONE
    // co_rep               -> DONE

    // Test SYM_CUBE        -> DONE
    // Test INV_IDX         -> DONE
    // Test MULT_SYM        -> DONE
    // Test CONJ_MOVE       -> DONE

    // Test get_flipslice_twist_depth3  -> DONE
    // Test get_corners_ud_edges_depth3 -> DONE
}
