use SciComp_project::rubiks::cube::RubiksCube;
use SciComp_project::cfop::f2l::solve_f2l;

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

fn f2l_solved() {
    let solved_cube = RubiksCube::new();
    let mut tested_cube = RubiksCube::new();
    // get the color of the bottom
    let bottom = solved_cube.faces[1][0];
    // Test various moves...
    // let scramble = "U L U' L'";
    let scramble = "U F U' F'";
    tested_cube.apply_scramble(&flip_and_toggle_moves(scramble));

    // perform f2l to solve cube
    solve_f2l(&mut tested_cube, &bottom);
    assert_eq!(solved_cube, tested_cube);
}
 