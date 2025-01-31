use two_phase::gui::app::CubeVisualizerWithMoves;
use two_phase::rubiks::cubie::{generate_states, CubieCube};
use two_phase::rubiks::performance::{self};

fn main() {
    // Test performance BFS vs IDA*
    // for n in 0..20 {
    //    if let Err(e) = performance::compare_algorithms(n) {
    //         eprintln!("Error: {}", e);
    //     }
    // }

    // Measure IDA* performance
    for n in 0..20 {
        if let Err(e) = performance::measure_ida(n) {
            eprintln!("Error: {}", e);
        }
    }

    // Measure BFS performance
    //or n in 0..20 {
    //   if let Err(e) = performance::measure_bfs(n) {
    //       eprintln!("Error: {}", e);
    //   }
    //

    // Measure two phase performance
    /*
    if let Err(e) = performance::measure_two_phase_ida() {
        eprintln!("Error: {}", e);
    }

    if let Err(e) = performance::measure_two_phase() {
        eprintln!("Error: {}", e);
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
    */
}
