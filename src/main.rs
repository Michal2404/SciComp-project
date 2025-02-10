use two_phase::gui::app::CubeVisualizerWithMoves;
use two_phase::rubiks_two_phase::cubie::{generate_states, CubieCube};
use two_phase::rubiks_two_phase::moves::TWIST_MOVE;
use two_phase::rubiks_two_phase::performance;
use two_phase::rubiks_two_phase::pruning::{get_flipslice_twist_depth3, FLIPSLICE_TWIST_DEPTH3};

fn main() {
    // Performance tests

    /*
        // Measure IDA* performance

        if let Err(e) = performance::measure_ida() {
            eprintln!("Error: {}", e);
        }

        // Measure BFS performance
        if let Err(e) = performance::measure_bfs() {
            eprintln!("Error: {}", e);
        }

        // Measure two phase performance

        if let Err(e) = performance::measure_two_phase_ida() {
            eprintln!("Error: {}", e);
        }

    if let Err(e) = performance::measure_two_phase() {
        eprintln!("Error: {}", e);
    }

        if let Err(e) = performance::measure_ida_depth_performance() {
            eprintln!("Error: {}", e);
        }

        // Measure two_phase time and length
        if let Err(e) = performance::two_phase_len_performance() {
            eprintln!("Error: {}", e);
        }
    */

    // Run the app

    let step = 18;
    let batch_size = 5;
    let times = 7;

    for i in 0..times {
        let start = i * step;
        let end = start + batch_size;

        if end <= TWIST_MOVE.len() {
            println!("Batch {}: {:?}", i + 1, &TWIST_MOVE[start..end]);
        } else {
            println!("Batch {}: Out of bounds!", i + 1);
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
}
