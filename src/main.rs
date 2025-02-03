use two_phase::gui::app::CubeVisualizerWithMoves;
use two_phase::rubiks::cubie::{generate_states, CubieCube};
//use two_phase::rubiks::performance;

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
