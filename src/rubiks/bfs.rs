use std::{
    collections::{HashSet, VecDeque},
    io::{self},
    time::Instant,
};

use once_cell::sync::Lazy;

use super::{
    cubie::CubieCube,
    enums::{to_move, Move},
};

pub fn bfs_solver(cube: &CubieCube, max_depth: usize) -> Option<Vec<Move>> {
    let start = Instant::now();
    let mut queue: VecDeque<(CubieCube, Vec<Move>)> = VecDeque::new();
    let mut visited: HashSet<CubieCube> = HashSet::new();
    // Start with the inital cube stat and no moves applied
    queue.push_back((cube.clone(), vec![]));
    visited.insert(cube.clone());

    while let Some((current_cube, current_moves)) = queue.pop_front() {
        // Check if the current cube is solved
        if current_cube.is_solved() {
            return Some(current_moves);
        }

        // If we reached the depth limit, skip further exploration
        if current_moves.len() >= max_depth {
            continue;
        }

        // Generate and enqueue all possible moves
        for &m in &Move::ALL {
            let permutation_cube = CubieCube::from_scramble(m.name());
            let mut next_cube = current_cube.clone();
            next_cube.multiply(&permutation_cube);
            if visited.insert(next_cube.clone()) {
                let mut next_moves = current_moves.clone();
                next_moves.push(m);
                queue.push_back((next_cube, next_moves));
            }
        }
    }
    println!("No solution found, elapsed time: {:?}", start.elapsed());
    // No solution found within the given depth
    None
}

pub fn ida_star_solver(cube: &CubieCube, max_depth: usize) -> Option<Vec<Move>> {
    let mut threshold = cube.heuristic();
    let mut path = vec![];
    let mut next_threshold = usize::MAX;

    while threshold <= max_depth {
        let result = ida_star_search(&cube, threshold, &mut path, &mut next_threshold, 0);

        if let Some(solution) = result {
            return Some(simplify_solution(&solution));
        }

        // Update the threshold to the next smallest cost greater than the current threshold.
        if next_threshold == usize::MAX {
            break;
        }
        threshold = next_threshold;
        next_threshold = usize::MAX;
    }
    None // No solution found within max_depth
}

// Recursive DFS with cost threshold for IDA*
pub fn ida_star_search(
    cube: &CubieCube,
    threshold: usize,
    path: &mut Vec<Move>,
    next_threshold: &mut usize,
    g: usize,
) -> Option<Vec<Move>> {
    let f = g + cube.heuristic();

    // If the estimated cost exceeds the current threshold, return
    if f > threshold {
        *next_threshold = (*next_threshold).min(f);
        return None;
    }

    // If the cube is solved, return the solution path
    if cube.is_solved() {
        return Some(path.clone());
    }

    // Explore all possivle moves
    for &m in &Move::ALL {
        // Avoid undoing the last move
        if let Some(&last_move) = path.last() {
            if last_move.invert() == m {
                continue;
            }
        }

        // Apply the move and search recursively
        let permutation_cube = CubieCube::from_scramble(m.name());
        let mut next_cube = cube.clone();
        next_cube.multiply(&permutation_cube);
        path.push(m);

        if let Some(solution) = ida_star_search(&next_cube, threshold, path, next_threshold, g + 1)
        {
            return Some(solution);
        }

        // Backtrack
        path.pop();
    }

    None
}

pub fn simplify_solution(solution: &[Move]) -> Vec<Move> {
    let mut simplified: Vec<Move> = Vec::new();

    for &move_ in solution {
        if let Some(&last) = simplified.last() {
            // If the current move is on the same face as the last move
            if last.face() == move_.face() {
                let new_move = match (last.turns() + move_.turns()) % 4 {
                    0 => None,                          // Cancel out (identity move)
                    1 => Some(to_move(last.face(), 1)), // One turn
                    2 => Some(to_move(last.face(), 2)), // Two turns
                    3 => Some(to_move(last.face(), 3)), // Three turns
                    _ => unreachable!(),
                };
                // Replace the last move or remove if it cancels out
                simplified.pop();
                if let Some(new_move) = new_move {
                    simplified.push(new_move);
                }
            } else {
                // Different face, push the move
                simplified.push(move_);
            }
        } else {
            // No previous move, push the move
            simplified.push(move_);
        }
    }

    simplified
}

pub static CORNER_DB: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut database = vec![u8::MAX; 8 * 3 * 8]; // 8 corners * 3 orientations * 8 permutations
    let mut queue: VecDeque<(CubieCube, usize)> = VecDeque::new();
    let mut visited: HashSet<CubieCube> = HashSet::new();
    let max_depth = 3;

    let solved_cube = CubieCube::new(None, None, None, None);
    queue.push_back((solved_cube.clone(), 0));
    visited.insert(solved_cube);

    while let Some((current_cube, depth)) = queue.pop_front() {
        if depth >= max_depth {
            break;
        }
        // Iterate through all corner positions
        for (corner_index, (cp, co)) in current_cube.cp.iter().zip(&current_cube.co).enumerate() {
            let permutation = *cp as usize;
            let orientation = *co as usize;

            let index = permutation * 3 * 8 + corner_index * 3 + orientation;
            database[index] = database[index].min(depth as u8);
        }
        // Explore all possible moves
        for &m in &Move::ALL {
            let permutation_cube = CubieCube::from_scramble(m.name());
            let mut next_cube = current_cube.clone();
            next_cube.multiply(&permutation_cube);

            if visited.insert(next_cube.clone()) {
                queue.push_back((next_cube, depth + 1));
            }
        }
    }
    database
});

pub static EDGE_DB: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut database = vec![u8::MAX; 12 * 2 * 12]; // 12 edges * 2 orientations * 12 permutations
    let mut queue: VecDeque<(CubieCube, usize)> = VecDeque::new();
    let mut visited: HashSet<CubieCube> = HashSet::new();
    let max_depth = 4;

    let solved_cube = CubieCube::new(None, None, None, None);
    queue.push_back((solved_cube.clone(), 0));
    visited.insert(solved_cube);

    while let Some((current_cube, depth)) = queue.pop_front() {
        if depth >= max_depth {
            break;
        }
        // Iterate through all edge positions
        for (edge_index, (ep, eo)) in current_cube.ep.iter().zip(&current_cube.eo).enumerate() {
            let permutation = *ep as usize;
            let orientation = *eo as usize;

            let index = permutation * 2 * 12 + edge_index * 2 + orientation;
            database[index] = database[index].min(depth as u8);
        }
        // Explore all possible moves
        for &m in &Move::ALL {
            let permutation_cube = CubieCube::from_scramble(m.name());
            let mut next_cube = current_cube.clone();
            next_cube.multiply(&permutation_cube);

            if visited.insert(next_cube.clone()) {
                queue.push_back((next_cube, depth + 1));
            }
        }
    }

    database
});

pub fn load_edges_databsae(filename: &str) -> io::Result<Vec<u8>> {
    use std::fs;
    let data = fs::read(filename)?;
    Ok(data)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_bfs_solver() {
        let cube = CubieCube::from_scramble("R U L B F D");
        let max_depth = 6;
        let moves = bfs_solver(&cube, max_depth).unwrap();
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn test_corner_db() {
        assert_eq!(CORNER_DB.len(), 8 * 3 * 8);
        assert_eq!(CORNER_DB[8], 2);
    }

    #[test]
    fn test_edge_db() {
        assert_eq!(EDGE_DB.len(), 12 * 2 * 12);
        assert_eq!(CORNER_DB[7], 2);
    }

    #[test]
    fn test_ida_solver() {
        let cube = CubieCube::from_scramble("R U L B F D R2 B2");
        let max_depth = 20;
        let moves = ida_star_solver(&cube, max_depth).unwrap();
        let simplified = simplify_solution(&moves);
        assert_eq!(simplified.len(), 8);
    }
}
