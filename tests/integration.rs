use rubiks::rubiks::cube::RubiksCube;

#[test]
fn is_solved() {
    let solved_cube = RubiksCube::new();
    let mut tested_cube = RubiksCube::new();
    // Test various moves...
    let scramble = "R2 L2 U2 D2 F2 B2 D2 U2 F2 B2 R2 L2";
    tested_cube.apply_scramble(scramble);
    assert_eq!(solved_cube, tested_cube);
}
