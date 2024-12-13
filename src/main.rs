use eframe::NativeOptions;
use rubiks::rubiks::cube::RubiksCube;
use rubiks::ui::app::MyApp;

fn main() {
    // Create new instance of the Cube
    let mut cube = RubiksCube::new();
    // Define the scramble in the standard notation
    let scramble = "U R F2";
    // Scramble the Cube
    cube.apply_scramble(scramble);

    // Visualize scrambled cube
    let options = NativeOptions::default();
    let _ = eframe::run_native(
        "Rubik's Cube Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(cube)))),
    );
}
