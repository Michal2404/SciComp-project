use SciComp_project::rubiks::cube::RubiksCube;
use SciComp_project::cfop::f2l::solve_f2l;
use SciComp_project::cfop::oll::solve_oll;

fn flip_and_toggle_moves(input: &str) -> String {
    input
        .split_whitespace() // Split the input into moves
        .rev() // Reverse the order of moves
        .map(|move_| {
            if move_.ends_with('\'') {
                // Remove the trailing `'` if it exists
                move_.trim_end_matches('\'').to_string()
            } else {
                // Add `'` if it doesn't exist
                format!("{}'", move_)
            }
        })
        .collect::<Vec<_>>() // Collect into a vector of strings
        .join(" ") // Join back into a single string
}



#[test]
// fn is_solved() {
//     let solved_cube = RubiksCube::new();
//     let mut tested_cube = RubiksCube::new();
//     // Test various moves...
//     let scramble = "R2 L2 U2 D2 F2 B2 D2 U2 F2 B2 R2 L2";
//     tested_cube.apply_scramble(scramble);
//     assert_eq!(solved_cube, tested_cube);
// }

// fn f2l_solved() {
//     let solved_cube = RubiksCube::new();
//     let mut tested_cube = RubiksCube::new();
//     // get the color of the bottom
//     let bottom = solved_cube.faces[1][0];
//     // Test various moves...
//     let scramble = "R' U R' U' F' U F R R";
//     tested_cube.apply_scramble(&flip_and_toggle_moves(scramble));

//     // perform f2l to solve cube
//     let moves = solve_f2l(&mut tested_cube, &bottom);
//     assert_eq!((solved_cube, scramble), (tested_cube, moves.join(" ").as_str()));
// }

fn oll_solved() {
    let solved_cube = RubiksCube::new();
    let mut tested_cube = RubiksCube::new();
    // get the color of the bottom
    let top = solved_cube.faces[0][0];
    // Test various moves...
    let scramble = "U U R U R' U' R' F R F' L' U' L U L F' L' F";
    tested_cube.apply_scramble(&flip_and_toggle_moves(scramble));

    // perform f2l to solve cube
    let moves = solve_oll(&mut tested_cube, &top);
    assert_eq!((solved_cube, scramble), (tested_cube, moves.join(" ").as_str()));
}