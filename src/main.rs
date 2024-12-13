use eframe::NativeOptions;
use rubiks::rubiks::cube::RubiksCube;
use rubiks::ui::app::MyApp;

fn main() {
    let mut cube = RubiksCube::new();
    let scramble = "U R F2";
    cube.apply_scramble(scramble);

    let options = NativeOptions::default();
    let _ = eframe::run_native(
        "Rubik's Cube Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(cube)))),
    );
}
