use super::cube::RubiksCube;
use std::collections::{HashSet, VecDeque};

//TODO: Implement the solver
impl RubiksCube {
    pub fn solve(&self) -> Option<Vec<String>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // Start BFS with the initial state
        queue.push_back((self.clone(), vec![])); // (current state, moves to reach it)
        visited.insert(self.to_string());

        while let Some((mut current_cube, path)) = queue.pop_front() {
            // Check if the cube is solved
            if current_cube.is_solved() {
                return Some(path); // Return the sequence of moves
            }

            // Explore all possible moves
            for (move_name, move_fn) in current_cube.all_moves() {
                let mut next_cube = current_cube.clone();
                move_fn(&mut next_cube);

                // Serialize the state for comparison
                let next_state = next_cube.to_string();
                if !visited.contains(&next_state) {
                    visited.insert(next_state);
                    let mut new_path = path.clone();
                    new_path.push(move_name.to_string());
                    queue.push_back((next_cube, new_path));
                }
            }
        }

        None // No solution found (shouldn't happen for a valid Rubik's Cube)
    }
}
