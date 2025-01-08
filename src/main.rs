use two_phase::gui::app::CubeVisualizer;
use two_phase::rubiks::cubie::{self, CubieCube};
use two_phase::rubiks::moves;
fn main() {
    // Create scrambled Cube
    let scramble = "F2 R2 D L2 D R2 F2 D L2 D L2 D2 B' R' F2 L2 B R2 D2 L2 F'";
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

    // Visualize the scramble
    let app = CubeVisualizer::new(scrambled.to_facelet_cube());
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Cube Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    );
}
