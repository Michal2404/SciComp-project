use SciComp_project::rubiks::cube::RubiksCube;
use SciComp_project::cfop::f2l::solve_f2l;
use SciComp_project::cfop::oll::solve_oll;
use SciComp_project::cfop::pll::solve_pll;

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
//     let scramble = "M M U M' U U M U M M";
//     // let scramble = "M M";
//     tested_cube.apply_scramble(&flip_and_toggle_moves(&scramble));
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

// fn oll_solved() {
//     let solved_cube = RubiksCube::new();
//     let mut tested_cube = RubiksCube::new();
//     // get the color of the bottom
//     let top = solved_cube.faces[0][0];
//     // Test various moves...
//     let scramble = "R L' U R' U' L R' F R F'";
//     tested_cube.apply_scramble(&flip_and_toggle_moves(scramble));

//     // perform f2l to solve cube
//     let moves = solve_oll(&mut tested_cube, &top);
//     assert_eq!((solved_cube, scramble), (tested_cube, moves.join(" ").as_str()));
// }

fn pll_solved() {
    let solved_cube = RubiksCube::new();
    let mut tested_cube = RubiksCube::new();
    // Test various moves...
    let scramble = "R U R' U' R' F R R U' R' U' R U R' F'";
    tested_cube.apply_scramble(&flip_and_toggle_moves(scramble));
    // tested_cube.apply_scramble(scramble);

    // perform f2l to solve cube
    let moves = solve_pll(&mut tested_cube);
    assert_eq!((solved_cube, scramble), (tested_cube, moves.join(" ").as_str()));
}