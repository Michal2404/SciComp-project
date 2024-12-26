use super::color::Color;
use crate::ui::app::MyApp;
use eframe::NativeOptions;

// Define main data structure for the cube
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RubiksCube {
    pub faces: [[Color; 9]; 6], // 6 faces, each with 9 stickers
}

impl RubiksCube {
    // Create new cube in the solved state
    pub fn new() -> Self {
        RubiksCube {
            faces: [
                [Color::W; 9], // Top face
                [Color::Y; 9], // Bottom face
                [Color::G; 9], // Front face
                [Color::B; 9], // Back face
                [Color::R; 9], // Right face
                [Color::O; 9], // Left face
            ],
        }
    }

    pub fn to_string(&self) -> String {
        self.faces
            .iter()
            .flat_map(|face| face.iter())
            .map(|color| match color {
                Color::W => 'W',
                Color::Y => 'Y',
                Color::G => 'G',
                Color::B => 'B',
                Color::R => 'R',
                Color::O => 'O',
            })
            .collect()
    }

    pub fn all_moves(&mut self) -> Vec<(&'static str, fn(&mut Self))> {
        vec![
            ("U", RubiksCube::u_clockwise),
            ("U'", RubiksCube::u_counterclockwise),
            ("D", RubiksCube::d_clockwise),
            ("D'", RubiksCube::d_counterclockwise),
            ("F", RubiksCube::f_clockwise),
            ("F'", RubiksCube::f_counterclockwise),
            ("B", RubiksCube::b_clockwise),
            ("B'", RubiksCube::b_counterclockwise),
            ("L", RubiksCube::l_clockwise),
            ("L'", RubiksCube::l_counterclockwise),
            ("R", RubiksCube::r_clockwise),
            ("R'", RubiksCube::r_counterclockwise),
        ]
    }

    // Apple scramble to the sube using the standard notation:
    // R:  move right face clockwise
    // R': move right face counterclockwise
    // Same for L: left, U: up, D: down, F: front, B: back
    pub fn apply_scramble(&mut self, scramble: &str) {
        let mut chars = scramble.chars().peekable();

        while let Some(c) = chars.next() {
            // Determine the move
            let move_fn = match c {
                'U' => Self::u_clockwise,
                'D' => Self::d_clockwise,
                'F' => Self::f_clockwise,
                'B' => Self::b_clockwise,
                'L' => Self::l_clockwise,
                'R' => Self::r_clockwise,
                _ => continue, // Skip invalid characters
            };

            // Check for modifiers (e.g., `'` or `2`)
            if let Some(&next) = chars.peek() {
                match next {
                    '\'' => {
                        chars.next(); // Consume the `'`
                        match c {
                            'U' => Self::u_counterclockwise(self),
                            'D' => Self::d_counterclockwise(self),
                            'F' => Self::f_counterclockwise(self),
                            'B' => Self::b_counterclockwise(self),
                            'L' => Self::l_counterclockwise(self),
                            'R' => Self::r_counterclockwise(self),
                            _ => {}
                        }
                        continue;
                    }
                    '2' => {
                        chars.next(); // Consume the `2`
                        move_fn(self); // Apply once
                        move_fn(self); // Apply again
                        continue;
                    }
                    _ => {}
                }
            }

            // Apply the move normally
            move_fn(self);
        }
    }

    // Check if cube is in solved state
    pub fn is_solved(&self) -> bool {
        let solved_cube = RubiksCube::new();
        if solved_cube == *self {
            return true;
        }
        false
    }

    // Apply one of the 12 possible moves to the cube by
    // changing positions of single stickers
    pub fn u_clockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[0][0] = temp[0][6];
        self.faces[0][1] = temp[0][3];
        self.faces[0][2] = temp[0][0];
        self.faces[0][3] = temp[0][7];
        self.faces[0][5] = temp[0][1];
        self.faces[0][6] = temp[0][8];
        self.faces[0][7] = temp[0][5];
        self.faces[0][8] = temp[0][2];
        self.faces[2][0] = temp[4][0];
        self.faces[2][1] = temp[4][1];
        self.faces[2][2] = temp[4][2];
        self.faces[3][0] = temp[5][0];
        self.faces[3][1] = temp[5][1];
        self.faces[3][2] = temp[5][2];
        self.faces[4][0] = temp[3][0];
        self.faces[4][1] = temp[3][1];
        self.faces[4][2] = temp[3][2];
        self.faces[5][0] = temp[2][0];
        self.faces[5][1] = temp[2][1];
        self.faces[5][2] = temp[2][2];
    }

    pub fn u_counterclockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[0][0] = temp[0][2];
        self.faces[0][1] = temp[0][5];
        self.faces[0][2] = temp[0][8];
        self.faces[0][3] = temp[0][1];
        self.faces[0][5] = temp[0][7];
        self.faces[0][6] = temp[0][0];
        self.faces[0][7] = temp[0][3];
        self.faces[0][8] = temp[0][6];
        self.faces[2][0] = temp[5][0];
        self.faces[2][1] = temp[5][1];
        self.faces[2][2] = temp[5][2];
        self.faces[3][0] = temp[4][0];
        self.faces[3][1] = temp[4][1];
        self.faces[3][2] = temp[4][2];
        self.faces[4][0] = temp[2][0];
        self.faces[4][1] = temp[2][1];
        self.faces[4][2] = temp[2][2];
        self.faces[5][0] = temp[3][0];
        self.faces[5][1] = temp[3][1];
        self.faces[5][2] = temp[3][2];
    }

    pub fn d_clockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[1][0] = temp[1][6];
        self.faces[1][1] = temp[1][3];
        self.faces[1][2] = temp[1][0];
        self.faces[1][3] = temp[1][7];
        self.faces[1][5] = temp[1][1];
        self.faces[1][6] = temp[1][8];
        self.faces[1][7] = temp[1][5];
        self.faces[1][8] = temp[1][2];
        self.faces[2][6] = temp[5][6];
        self.faces[2][7] = temp[5][7];
        self.faces[2][8] = temp[5][8];
        self.faces[3][6] = temp[4][6];
        self.faces[3][7] = temp[4][7];
        self.faces[3][8] = temp[4][8];
        self.faces[4][6] = temp[2][6];
        self.faces[4][7] = temp[2][7];
        self.faces[4][8] = temp[2][8];
        self.faces[5][6] = temp[3][6];
        self.faces[5][7] = temp[3][7];
        self.faces[5][8] = temp[3][8];
    }

    pub fn d_counterclockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[1][0] = temp[1][2];
        self.faces[1][1] = temp[1][5];
        self.faces[1][2] = temp[1][8];
        self.faces[1][3] = temp[1][1];
        self.faces[1][5] = temp[1][7];
        self.faces[1][6] = temp[1][0];
        self.faces[1][7] = temp[1][3];
        self.faces[1][8] = temp[1][6];
        self.faces[2][6] = temp[4][6];
        self.faces[2][7] = temp[4][7];
        self.faces[2][8] = temp[4][8];
        self.faces[3][6] = temp[5][6];
        self.faces[3][7] = temp[5][7];
        self.faces[3][8] = temp[5][8];
        self.faces[4][6] = temp[3][6];
        self.faces[4][7] = temp[3][7];
        self.faces[4][8] = temp[3][8];
        self.faces[5][6] = temp[2][6];
        self.faces[5][7] = temp[2][7];
        self.faces[5][8] = temp[2][8];
    }

    pub fn f_clockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[2][0] = temp[2][6];
        self.faces[2][1] = temp[2][3];
        self.faces[2][2] = temp[2][0];
        self.faces[2][3] = temp[2][7];
        self.faces[2][5] = temp[2][1];
        self.faces[2][6] = temp[2][8];
        self.faces[2][7] = temp[2][5];
        self.faces[2][8] = temp[2][2];
        self.faces[0][6] = temp[5][8];
        self.faces[0][7] = temp[5][5];
        self.faces[0][8] = temp[5][2];
        self.faces[1][0] = temp[4][6];
        self.faces[1][1] = temp[4][3];
        self.faces[1][2] = temp[4][0];
        self.faces[4][0] = temp[0][6];
        self.faces[4][3] = temp[0][7];
        self.faces[4][6] = temp[0][8];
        self.faces[5][2] = temp[1][0];
        self.faces[5][5] = temp[1][1];
        self.faces[5][8] = temp[1][2];
    }

    pub fn f_counterclockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[2][0] = temp[2][2];
        self.faces[2][1] = temp[2][5];
        self.faces[2][2] = temp[2][8];
        self.faces[2][3] = temp[2][1];
        self.faces[2][5] = temp[2][7];
        self.faces[2][6] = temp[2][0];
        self.faces[2][7] = temp[2][3];
        self.faces[2][8] = temp[2][6];
        self.faces[0][6] = temp[4][0];
        self.faces[0][7] = temp[4][3];
        self.faces[0][8] = temp[4][6];
        self.faces[1][0] = temp[5][2];
        self.faces[1][1] = temp[5][5];
        self.faces[1][2] = temp[5][8];
        self.faces[4][0] = temp[1][2];
        self.faces[4][3] = temp[1][1];
        self.faces[4][6] = temp[1][0];
        self.faces[5][2] = temp[0][8];
        self.faces[5][5] = temp[0][7];
        self.faces[5][8] = temp[0][6];
    }

    pub fn b_clockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[3][0] = temp[3][6];
        self.faces[3][1] = temp[3][3];
        self.faces[3][2] = temp[3][0];
        self.faces[3][3] = temp[3][7];
        self.faces[3][5] = temp[3][1];
        self.faces[3][6] = temp[3][8];
        self.faces[3][7] = temp[3][5];
        self.faces[3][8] = temp[3][2];
        self.faces[0][0] = temp[4][2];
        self.faces[0][1] = temp[4][5];
        self.faces[0][2] = temp[4][8];
        self.faces[1][6] = temp[5][0];
        self.faces[1][7] = temp[5][3];
        self.faces[1][8] = temp[5][6];
        self.faces[4][2] = temp[1][8];
        self.faces[4][5] = temp[1][7];
        self.faces[4][8] = temp[1][6];
        self.faces[5][0] = temp[0][2];
        self.faces[5][3] = temp[0][1];
        self.faces[5][6] = temp[0][0];
    }

    pub fn b_counterclockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[3][0] = temp[3][2];
        self.faces[3][1] = temp[3][5];
        self.faces[3][2] = temp[3][8];
        self.faces[3][3] = temp[3][1];
        self.faces[3][5] = temp[3][7];
        self.faces[3][6] = temp[3][0];
        self.faces[3][7] = temp[3][3];
        self.faces[3][8] = temp[3][6];
        self.faces[0][0] = temp[5][6];
        self.faces[0][1] = temp[5][3];
        self.faces[0][2] = temp[5][0];
        self.faces[1][6] = temp[4][8];
        self.faces[1][7] = temp[4][5];
        self.faces[1][8] = temp[4][2];
        self.faces[4][2] = temp[0][0];
        self.faces[4][5] = temp[0][1];
        self.faces[4][8] = temp[0][2];
        self.faces[5][0] = temp[1][6];
        self.faces[5][3] = temp[1][7];
        self.faces[5][6] = temp[1][8];
    }

    pub fn r_clockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[4][0] = temp[4][6];
        self.faces[4][1] = temp[4][3];
        self.faces[4][2] = temp[4][0];
        self.faces[4][3] = temp[4][7];
        self.faces[4][5] = temp[4][1];
        self.faces[4][6] = temp[4][8];
        self.faces[4][7] = temp[4][5];
        self.faces[4][8] = temp[4][2];
        self.faces[0][2] = temp[2][2];
        self.faces[0][5] = temp[2][5];
        self.faces[0][8] = temp[2][8];
        self.faces[1][2] = temp[3][6];
        self.faces[1][5] = temp[3][3];
        self.faces[1][8] = temp[3][0];
        self.faces[2][2] = temp[1][2];
        self.faces[2][5] = temp[1][5];
        self.faces[2][8] = temp[1][8];
        self.faces[3][0] = temp[0][8];
        self.faces[3][3] = temp[0][5];
        self.faces[3][6] = temp[0][2];
    }

    pub fn r_counterclockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[4][0] = temp[4][2];
        self.faces[4][1] = temp[4][5];
        self.faces[4][2] = temp[4][8];
        self.faces[4][3] = temp[4][1];
        self.faces[4][5] = temp[4][7];
        self.faces[4][6] = temp[4][0];
        self.faces[4][7] = temp[4][3];
        self.faces[4][8] = temp[4][6];
        self.faces[0][2] = temp[3][6];
        self.faces[0][5] = temp[3][3];
        self.faces[0][8] = temp[3][0];
        self.faces[1][2] = temp[2][2];
        self.faces[1][5] = temp[2][5];
        self.faces[1][8] = temp[2][8];
        self.faces[2][2] = temp[0][2];
        self.faces[2][5] = temp[0][5];
        self.faces[2][8] = temp[0][8];
        self.faces[3][0] = temp[1][8];
        self.faces[3][3] = temp[1][5];
        self.faces[3][6] = temp[1][2];
    }

    pub fn l_clockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[5][0] = temp[5][6];
        self.faces[5][1] = temp[5][3];
        self.faces[5][2] = temp[5][0];
        self.faces[5][3] = temp[5][7];
        self.faces[5][5] = temp[5][1];
        self.faces[5][6] = temp[5][8];
        self.faces[5][7] = temp[5][5];
        self.faces[5][8] = temp[5][2];
        self.faces[0][0] = temp[3][8];
        self.faces[0][3] = temp[3][5];
        self.faces[0][6] = temp[3][2];
        self.faces[1][0] = temp[2][0];
        self.faces[1][3] = temp[2][3];
        self.faces[1][6] = temp[2][6];
        self.faces[2][0] = temp[0][0];
        self.faces[2][3] = temp[0][3];
        self.faces[2][6] = temp[0][6];
        self.faces[3][2] = temp[1][6];
        self.faces[3][5] = temp[1][3];
        self.faces[3][8] = temp[1][0];
    }

    pub fn l_counterclockwise(&mut self) {
        let temp = self.faces.clone();
        self.faces[5][0] = temp[5][2];
        self.faces[5][1] = temp[5][5];
        self.faces[5][2] = temp[5][8];
        self.faces[5][3] = temp[5][1];
        self.faces[5][5] = temp[5][7];
        self.faces[5][6] = temp[5][0];
        self.faces[5][7] = temp[5][3];
        self.faces[5][8] = temp[5][6];
        self.faces[0][0] = temp[2][0];
        self.faces[0][3] = temp[2][3];
        self.faces[0][6] = temp[2][6];
        self.faces[1][0] = temp[3][8];
        self.faces[1][3] = temp[3][5];
        self.faces[1][6] = temp[3][2];
        self.faces[2][0] = temp[1][0];
        self.faces[2][3] = temp[1][3];
        self.faces[2][6] = temp[1][6];
        self.faces[3][2] = temp[0][6];
        self.faces[3][5] = temp[0][3];
        self.faces[3][8] = temp[0][0];
    }

    pub fn visualize(self) {
        let options = NativeOptions::default();
        let cube_clone = self.clone();
        let _ = eframe::run_native(
            "Rubik's Cube Visualizer",
            options,
            Box::new(|_cc| Ok(Box::new(MyApp::new(cube_clone)))),
        );
    }
}

// For visualization
impl std::fmt::Display for RubiksCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, face) in self.faces.iter().enumerate() {
            writeln!(f, "Face {}: {:?}", i, face)?;
        }
        Ok(())
    }
}
