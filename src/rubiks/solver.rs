use std::time::Instant;

use crate::rubiks::bfs::ida_star_solver;
use crate::rubiks::face::FaceCube;

use super::coord::{self, U_EDGES_PLUS_D_EDGES_TO_UD_EDGES};
use super::cubie;
use super::defs::N_FLIP;
use super::defs::N_MOVE;
use super::enums::Move;
use super::moves::{
    CORNERS_MOVE, D_EDGES_MOVE, FLIP_MOVE, SLICE_SORTED_MOVE, TWIST_MOVE, UD_EDGES_MOVE,
    U_EDGES_MOVE,
};
use super::pruning::{self as pr, CORNSLICE_DEPTH};
use super::symmetries::{
    CONJ_MOVE, CORNER_CLASSIDX, CORNER_SYM, FLIPSLICE_CLASSIDX, FLIPSLICE_SYM, SYM_CUBE,
    TWIST_CONJ, UD_EDGES_CONJ,
};

pub struct Solver {
    pub cb_cube: cubie::CubieCube, // Cube to be solved in CubieCube representation
    pub co_cube: Option<coord::CoordCube>, // Coordinate representation of the possible rotated/inverted cube
    pub rot: usize,                        // Rotation index (0, 1, 2)
    pub inv: usize,                        // Inversion flag (0 = no invert, 1 = invert)
    pub sofar_phase1: Vec<Move>,           // Moves so far in phase 1
    pub sofar_phase2: Vec<Move>,           // Moves so far in phase 2
    pub phase2_done: bool,                 // Phase 2 completion flag
    pub ret_length: usize,                 // Stop search if a solution of <= ret_length is found
    pub timeout: f64,                      // Max search time in seconds
    pub start_time: Instant,               // Start time of the search
    pub cornersave: usize,                 // Variable for saving corner state
    pub solutions: Vec<Vec<Move>>,         // All found solutions
    pub terminated: bool,                  // Termination signal
    pub shortest_length: usize,            // Length of the shortest solution found
}

impl Solver {
    pub fn new(
        cb_cube: cubie::CubieCube,
        rot: usize,
        inv: usize,
        ret_length: usize,
        timeout: f64,
        start_time: Instant,
    ) -> Self {
        Solver {
            cb_cube,
            co_cube: None,
            rot,
            inv,
            sofar_phase1: Vec::new(),
            sofar_phase2: Vec::new(),
            phase2_done: false,
            ret_length,
            timeout,
            start_time,
            cornersave: 0,
            solutions: Vec::new(),
            terminated: false,
            shortest_length: usize::MAX,
        }
    }

    pub fn search_phase2(
        &mut self,
        corners: usize,
        ud_edges: usize,
        slice_sorted: usize,
        dist: usize,
        togo_phase2: usize,
    ) {
        //println!("starting searcH_phase2");
        //println!("Moves in phase 1 so far: {:?}", self.sofar_phase1);

        // Check for termination or if phase 2 is already done
        if self.terminated || self.phase2_done {
            return;
        }

        // If phase 2 is complete
        if togo_phase2 == 0 && slice_sorted == 0 {
            // Contruct the entire maneuver = phase1 + phase2
            let mut man = Vec::new();
            man.extend_from_slice(&self.sofar_phase1);
            man.extend_from_slice(&self.sofar_phase2);

            // If no solutions yet or we found a strictly shorter one:
            if self.solutions.is_empty() || self.solutions.last().unwrap().len() > man.len() {
                // If we solved the inversed cube, reverse the moves and invert them
                if self.inv == 1 {
                    man.reverse();
                    for m in &mut man {
                        // R1 -> R3, R2 -> R2, R3 -> R1, etc.
                        *m = Move::from_id((m.id() / 3) * 3 + (2 - (m.id() % 3)));
                    }
                }

                // Apply conjugation by 'rot'
                for m in &mut man {
                    let idx = N_MOVE * 16 * self.rot + m.id();
                    *m = Move::from_id(CONJ_MOVE[idx] as usize);
                }

                // Store the solution and update shortest_length
                self.solutions.push(man.clone());
                self.shortest_length = man.len();
            }

            // If we reach or beat our target length, terminate
            if self.shortest_length <= self.ret_length {
                self.terminated = true;
            }

            // Mark phase2 as done for this branch
            self.phase2_done = true;
        } else {
            // Otherwise, continue searching
            for m in Move::iterator() {
                // Skip moves disallowed in phase2 from the original code:
                if matches!(
                    m,
                    Move::R1
                        | Move::R3
                        | Move::F1
                        | Move::F3
                        | Move::L1
                        | Move::L3
                        | Move::B1
                        | Move::B3
                ) {
                    continue;
                }

                // Disawllow successive moves on the same face or axis with improper order.
                if let Some(&last_move) = self.sofar_phase2.last() {
                    let diff = (last_move.id() / 3) as isize - (m.id() / 3) as isize;
                    if diff == 0 || diff == 3 {
                        continue;
                    }
                } else if let Some(&last_move) = self.sofar_phase1.last() {
                    let diff = (last_move.id() / 3) as isize - (m.id() / 3) as isize;
                    if diff == 0 || diff == 3 {
                        continue;
                    }
                }

                // Apply the moves to corners, edges, slices
                let corners_new = CORNERS_MOVE[18 * corners + m.id()] as usize;
                let ud_edges_new = UD_EDGES_MOVE[18 * ud_edges + m.id()] as usize;
                let slice_sorted_new = SLICE_SORTED_MOVE[18 * slice_sorted + m.id()];

                // Calculate distances
                let classidx = CORNER_CLASSIDX[corners_new];
                let sym = CORNER_SYM[corners_new];
                let dist_new_mod3 = pr::get_corners_ud_edges_depth3(
                    40320_usize * (classidx as usize)
                        + UD_EDGES_CONJ[(ud_edges_new << 4) + (sym as usize)] as usize,
                ) as usize;
                let dist_new = pr::DISTANCE[3 * dist + dist_new_mod3 as usize];
                // Check cornslice table for feasbility
                let depth_cornslice = CORNSLICE_DEPTH[24 * corners_new + slice_sorted_new as usize];
                if dist_new.max(depth_cornslice) >= togo_phase2 as i8 {
                    continue; // Not solvable within togo_phase2 - 1 moves
                }

                // Recurse
                self.sofar_phase2.push(m);
                self.search_phase2(
                    corners_new,
                    ud_edges_new,
                    slice_sorted_new as usize,
                    dist_new as usize,
                    togo_phase2 - 1,
                );
                self.sofar_phase2.pop();

                // Early termination check
                if self.terminated {
                    return;
                }
            }
        }
    }

    pub fn search_phase1(
        &mut self,
        flip: usize,
        twist: usize,
        slice_sorted: usize,
        dist: usize,
        togo_phase1: usize,
        depth_expanded: &mut usize,
    ) {
        // Print a checkpoint every 1000 expansions
        *depth_expanded += 1;

        // If we already terminated, do nothing:
        if self.terminated {
            return;
        }

        // If phase1 depth is reached (phase1 solved):
        if togo_phase1 == 0 {
            //println!("phase 1 solved, moves: {:?}", self.sofar_phase1);

            // Check timeout
            let elapsed = self.start_time.elapsed().as_secs_f64();
            if elapsed > self.timeout && !self.solutions.is_empty() {
                self.terminated = true;
                return;
            }

            // Compute initial phase2 coords
            let mut corners: usize;
            let last_move = self.sofar_phase1.last().copied().unwrap_or(Move::U1);

            if matches!(last_move, Move::R3 | Move::F3 | Move::L3 | Move::B3) {
                // corners = corners_move[18 * self.cornersave + (last_move.id() - 1)];
                // Because python uses R3 => R2? Actually, be careful. This logic is from your code.
                corners = CORNERS_MOVE[18 * self.cornersave + (last_move.id() - 1)] as usize;
            } else {
                corners = self.co_cube.as_ref().unwrap().corners;
                for &m in &self.sofar_phase1 {
                    corners = CORNERS_MOVE[18 * corners + m.id()] as usize;
                }
                self.cornersave = corners;
            }

            // Phase2 move limit
            let togo2_limit = (self.shortest_length - self.sofar_phase1.len()).min(11);

            // Pre-check with cornslice_depth
            let cornslice_val = CORNSLICE_DEPTH[24 * corners + slice_sorted];
            if cornslice_val as usize >= togo2_limit {
                return;
            }

            // Compute UD edges coordinate
            let mut u_edges = self.co_cube.as_ref().unwrap().u_edges;
            let mut d_edges = self.co_cube.as_ref().unwrap().d_edges;
            for &mov in &self.sofar_phase1 {
                let index = 18_usize * (u_edges as usize) + (mov.id());
                u_edges = U_EDGES_MOVE[index];

                let index2 = 18_usize * (d_edges as usize) + (mov.id());
                d_edges = D_EDGES_MOVE[index2];
            }

            let ud_edges =
                U_EDGES_PLUS_D_EDGES_TO_UD_EDGES[24 * u_edges as usize + (d_edges as usize % 24)];

            // Distance to phase2 start
            let dist2 = self
                .co_cube
                .as_ref()
                .unwrap()
                .get_depth_phase2(corners, ud_edges as usize);

            // Iterative deepening for phase2
            for togo2 in dist2..togo2_limit {
                self.sofar_phase2.clear();
                self.phase2_done = false;
                self.search_phase2(corners, ud_edges as usize, slice_sorted, dist2, togo2);
                if self.phase2_done || self.terminated {
                    break;
                }
            }
        } else {
            // Still in phase1. Explore moves.

            for m in Move::iterator() {
                // If dist == 0 => already in subgroup H. If fewer than 5 moves left,
                // forbid phase2 moves in phase1 (mirroring the original logic).
                if dist == 0
                    && togo_phase1 < 5
                    && matches!(
                        m,
                        Move::U1
                            | Move::U2
                            | Move::U3
                            | Move::R2
                            | Move::F2
                            | Move::D1
                            | Move::D2
                            | Move::D3
                            | Move::L2
                            | Move::B2
                    )
                {
                    continue;
                }

                // Disallow successive moves on same face or axis in the wrong order
                if let Some(&last_move) = self.sofar_phase1.last() {
                    let diff = (last_move.id() / 3) as isize - (m.id() / 3) as isize;
                    if diff == 0 || diff == 3 {
                        continue;
                    }
                }

                let flip_new = FLIP_MOVE[18 * flip + m.id()];
                let twist_new = TWIST_MOVE[18 * twist + m.id()] as usize;
                let slice_sorted_new = SLICE_SORTED_MOVE[18 * slice_sorted + m.id()];

                let flipslice = N_FLIP * (slice_sorted_new / 24) as usize + flip_new as usize; // N_FLIP * (slice//N_PERM_4) + flip
                let classidx = FLIPSLICE_CLASSIDX[flipslice];
                let sym = FLIPSLICE_SYM[flipslice];
                let dist_new_mod3 = pr::get_flipslice_twist_depth3(
                    2187_usize * (classidx as usize)
                        + TWIST_CONJ[(twist_new << 4) + (sym as usize)] as usize,
                );

                let dist_new = pr::DISTANCE[3 * dist + dist_new_mod3 as usize];
                if dist_new as usize >= togo_phase1 {
                    continue; // Not solvable in togo_phase1 - 1 moves
                }

                // Recurse deeper into phase1
                self.sofar_phase1.push(m);
                self.search_phase1(
                    flip_new as usize,
                    twist_new,
                    slice_sorted_new as usize,
                    dist_new as usize,
                    togo_phase1 - 1,
                    depth_expanded,
                );
                self.sofar_phase1.pop();

                // Early termination check
                if self.terminated {
                    return;
                }
            }
        }
    }
    /// The main solver routine (originally `run` in Python).
    pub fn run(&mut self) {
        let mut depth_expanded: usize = 0;

        // Rotate or invert the cube if needed, replicating the Python logic:

        let mut cb = match self.rot {
            0 => self.cb_cube, // no rotation
            1 => {
                let mut tmp = cubie::CubieCube::new(None, None, None, None); // placeholder
                                                                             // conj by 120° rotation: symCube[32] * cb_cube * symCube[16]
                tmp.clone_from(&SYM_CUBE[32]);
                tmp.multiply(&self.cb_cube);
                tmp.multiply(&SYM_CUBE[16]);
                tmp
            }
            2 => {
                let mut tmp = cubie::CubieCube::new(None, None, None, None); // placeholder
                                                                             // conj by 240° rotation: symCube[16] * cb_cube * symCube[32]
                tmp.clone_from(&SYM_CUBE[16]);
                tmp.multiply(&self.cb_cube);
                tmp.multiply(&SYM_CUBE[32]);
                tmp
            }
            _ => self.cb_cube,
        };

        if self.inv == 1 {
            let mut tmp = cubie::CubieCube::new(None, None, None, None);
            cb.inv_cubie_cube(&mut tmp);
            cb = tmp;
        }

        // Build the coord cube
        let co = coord::CoordCube::from_cubie_cube(&cb);
        self.co_cube = Some(co);

        // Phase1 iterative deepening
        let dist = self.co_cube.as_ref().unwrap().get_depth_phase1();
        for togo1 in dist..20 {
            self.sofar_phase1.clear();
            self.search_phase1(
                self.co_cube.as_ref().unwrap().flip as usize,
                self.co_cube.as_ref().unwrap().twist,
                self.co_cube.as_ref().unwrap().slice_sorted as usize,
                dist,
                togo1,
                &mut depth_expanded,
            );

            // If we found a solution or timed out, break early
            if self.terminated || !self.solutions.is_empty() {
                break;
            }
        }
    }
}

/// Solve a cube defined by its cube definition string (single-threaded).
///
/// - `cubestring`: a Facelet-based description of the cube (per your Python code).
/// - `max_length`: The maximum solution length to accept before stopping.
/// - `timeout`: The maximum search time (in seconds). If the solver times out, it returns the best found solution.
///
/// Returns a string of the solution (or an error message if invalid).
pub fn solve(
    cubescramble: &str,
    max_length: usize,
    timeout: f64,
    from_scramble: bool,
    use_ida: bool,
    ida_depth: Option<usize>,
) -> String {
    let cc = if from_scramble {
        cubie::CubieCube::from_scramble(cubescramble)
    } else {
        let mut fc = FaceCube::new();
        let _ = fc.from_string(cubescramble);
        fc.to_cubie_cube()
    };

    // 3) Start timing
    let start_time = Instant::now();

    // Use BFS to find candidate solutions up to depth n
    if use_ida {
        println!("Running IDA* for depth {}...", ida_depth.unwrap());
        if let Some(best_solution) = ida_star_solver(&cc, ida_depth.unwrap()) {
            println!("Solution found by IDA*: {:?}", best_solution);
            println!("Elapsed time: {:?}", start_time.elapsed());
            let mut s = String::new();
            for &mv in &best_solution {
                s.push_str(mv.name()); // or format!("{:?}", mv) if debug
                s.push(' ');
            }
            s = s.replace('3', "'").replace('1', "");
            let moves_count = best_solution.len();
            let formatted_string = format!("{}({}f)", s.trim_end(), moves_count);

            // Replace occurrences of '3' with '\''
            return formatted_string.to_string();
        }
        println!(
            "No solution found by IDA* after {:?}, proceeding to two-phase solver...",
            start_time.elapsed()
        );
    }

    let start_time_two_phase = Instant::now();
    let mut solver: Solver = Solver::new(cc, 0, 0, max_length, timeout, start_time);
    solver.run();
    println!("Total elapsed time: {:?}", start_time.elapsed());
    println!(
        "two-phase elapsed time: {:?}",
        start_time_two_phase.elapsed()
    );
    // 4) Construct the final solution string
    if !solver.solutions.is_empty() {
        // The last solution is presumably the shortest, by your code's convention
        let best_solution = solver.solutions.last().unwrap();
        let mut s = String::new();
        for mv in best_solution {
            s.push_str(mv.name()); // or format!("{:?}", mv) if debug
            s.push(' ');
        }
        s = s.replace('3', "'").replace('1', "");
        let moves_count = best_solution.len();
        let formatted_string = format!("{}({}f)", s.trim_end(), moves_count);

        // Replace occurrences of '3' with '\''
        formatted_string.to_string()
    } else {
        "No solution found.".to_string()
    }
}
