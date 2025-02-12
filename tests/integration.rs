use scicomp_project::cfop::f2l::solve_f2l;
use scicomp_project::rubiks::cube::RubiksCube;

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
// fn cross_solved() {
//     let solved_cube = RubiksCube::new();
//     let mut tested_cube = RubiksCube::new();
//     // Test various moves...
//     let scramble = "R U R' U' R' F R R U' R' U' R U R' F'";
//     // let scramble = "M M";
//     tested_cube.apply_scramble(&scramble);

//     // get the color of the bottom
//     let bottom = solved_cube.faces[1][0];
//     // now we solve the cross
//     let moves = solve_cross(&mut tested_cube, &bottom);

//     println!("{:?}", moves);

//     // tested_cube.apply_scramble(moves_str.as_str());
//     assert_eq!(solved_state(&tested_cube, &bottom).0, true)
// }

fn f2l_solved() {
    let solved_cube = RubiksCube::new();
    let mut tested_cube = RubiksCube::new();
    // get the color of the bottom
    let bottom = solved_cube.faces[1][0];
    // Test various moves...
    let scramble = "U' R U' R' U R U R'";
    tested_cube.apply_scramble(&flip_and_toggle_moves(scramble));
    // let scramble = "R U R' U' R' F R R U' R' U' R U R' F'";
    // tested_cube.apply_scramble(&scramble);

    // perform f2l to solve cube
    let moves = solve_f2l(&mut tested_cube, &bottom);
    println!("{:?}", moves);
    assert_eq!(
        (solved_cube, scramble),
        (tested_cube, moves.join(" ").as_str())
    );
}

// fn oll_solved() {
//     let solved_cube = RubiksCube::new();
//     let mut tested_cube = RubiksCube::new();
//     // get the color of the bottom
//     let top = solved_cube.faces[0][0];
//     // Test various moves...
//     // let scramble = "R L' U R' U' L R' F R F'";
//     // tested_cube.apply_scramble(&flip_and_toggle_moves(scramble));
//     let scramble = "R U R' U' R' F R R U' R' U' R U R' F'";
//     tested_cube.apply_scramble(&scramble);

//     // perform f2l to solve cube
//     let moves = solve_oll(&mut tested_cube, &top);
//     assert_eq!((solved_cube, scramble), (tested_cube, moves.join(" ").as_str()));
// }

// fn pll_solved() {
//     let solved_cube = RubiksCube::new();
//     let mut tested_cube = RubiksCube::new();
//     // Test various moves...
//     let scramble = "R L U U R' L' F' B' U U F B";
//     tested_cube.apply_scramble(&flip_and_toggle_moves(scramble));
//     // tested_cube.apply_scramble(scramble);

//     // perform f2l to solve cube
//     let moves = solve_pll(&mut tested_cube);
//     let cleaned_scramble: Vec<String> = cleanup_moves(scramble.split(" ").map(|s| s.to_string()).collect());
//     assert_eq!((solved_cube, cleaned_scramble), (tested_cube, moves));
// }

// fn solved() {
//     let solved_cube = RubiksCube::new();
//     let mut tested_cube = RubiksCube::new();
//     // Test various moves...
//     let scramble = "R U R' U' R' F R R U' R' U' R U R' F'";
//     // let scramble = "R U'";
//     tested_cube.apply_scramble(&scramble);

//     let mut tested_cube_clone = tested_cube.clone();

//     // perform f2l to solve cube
//     let (moves, _) = cfop_solver(&scramble, tested_cube);
//     println!("{}", &moves.join(" ").as_str());
//     tested_cube_clone.apply_scramble(&moves.join(" ").as_str());
//     assert_eq!(solved_cube, tested_cube_clone);

// }
