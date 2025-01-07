use two_phase::gui::app::CubeVisualizer;
use two_phase::rubiks::cubie::{self, CubieCube};
use two_phase::rubiks::moves;
fn main() {
    // Create scrambled Cube
    let scramble = "R";
    let scrambled = cubie::CubieCube::from_scramble(&scramble);
    println!("Scrambled cube in Cubie notation:");
    println!("{:?}\n", scrambled);

    // Make FaceCube from CubieCube
    let face_scrambled = scrambled.to_facelet_cube().to_2dstring();
    println!("Scrambled cube in Facelet notation:");
    println!("{}\n", face_scrambled);

    // Inverse the Cube
    let mut inverse = cubie::CubieCube::new(None, None, None, None);
    scrambled.inv_cubie_cube(&mut inverse);
    println!("Inverse of scrambled cube in Facelet notation:");
    println!("{}\n", inverse.to_facelet_cube().to_2dstring());

    // Corner parity of scrambled cube
    let corner_parity = scrambled.corner_parity();
    println!("Corner parity of scramlbed cube:");
    println!("{}\n", corner_parity);

    // Edge parity of scrambled cube
    let edge_parity = scrambled.edge_parity();
    println!("Edge parity of scramlbed cube:");
    println!("{}\n", edge_parity);

    // Corner orientation coordinate of the scrambled cube
    let corner_orientation_coord = scrambled.get_twist();
    println!("Corner orientation coordinate of scrambled cube (0<=twist<2187):");
    println!("{}\n", corner_orientation_coord);

    // Edge orientation coordinate of the scrambled cube
    let edge_orientation_coord = scrambled.get_flip();
    println!("Edge orientation coordinate of scrambled cube (0<=flip<2047):");
    println!("{}\n", edge_orientation_coord);

    // UD-slice edge coordinates of the scrambled cube
    let ud_slice_coord = scrambled.get_slice();
    println!("UD-slice coordinate of scrambeld cube (0<=slice<495)");
    println!("{}\n", ud_slice_coord);

    // Set corner twist
    //let mut twist: u16 = 10;
    //scrambled.set_twist(twist);

    //let mut parity_cube = cubie::CubieCube::new(None, None, None, None);
    //let mut twist: u16 = 27;
    //parity_cube.set_twist(twist);

    // get corner permutation coords
    let cube = CubieCube::from_scramble("R");
    let corner_perm_coord = cube.get_corners();
    println!("Corner permutation coords: {}", corner_perm_coord);

    // set corner permutation coords
    let cube = CubieCube::new(None, None, None, None);
    println!("{}", cube.get_corners());
    // cube.set_corners(21021);
    // println!("{}", cube.get_corners());

    /// Generate move table
    moves::generate_move_table();

    // Visualize the scramble
    let app = CubeVisualizer::new(scrambled.to_facelet_cube());
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Cube Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    );
}
