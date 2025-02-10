/// Here is the Cube representation of the coordinate level. The most important representation in the algorithm,
/// because it allows fast computation with the use of pre-generated tables.
use super::cubie as cb;
use super::defs;
use super::defs::{N_FLIP, N_MOVE, N_PERM_4, N_TWIST, N_UD_EDGES, N_U_EDGES_PHASE2};
use super::enums;
use super::moves::{
    CORNERS_MOVE, D_EDGES_MOVE, FLIP_MOVE, SLICE_SORTED_MOVE, TWIST_MOVE, UD_EDGES_MOVE,
    U_EDGES_MOVE,
};
use super::pruning::{self as pr};

use super::symmetries::{
    CORNER_CLASSIDX, CORNER_REP, CORNER_SYM, FLIPSLICE_CLASSIDX, FLIPSLICE_REP, FLIPSLICE_SYM,
    TWIST_CONJ, UD_EDGES_CONJ,
};
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::Read;

const SOLVED: usize = 0; // Index of the solved state

/// Represent a cube on the coordinate level.
/// In phase 1 state is uniquely determined by the three coordinates flip, twist and slice
/// In phsae 2 state is uniquely determined by the three coordinates corners, ud_edges and slice sorted % 24
pub struct CoordCube {
    pub twist: usize,      // Twist of corners
    pub flip: u16,         // Flip of Edges
    pub slice_sorted: u16, // Position of FR, FL, BL, BR edges (valid in phase 1 and 2)
    pub u_edges: u16,      // U edges coordinate (phase 1 and phase 2)
    pub d_edges: u16,      // D edges coordinate (phase 1 and phase 2)
    pub corners: usize,    // Corner permutation (phase 1 and phase 2)
    pub ud_edges: isize,   // Permutation of the UD edges (valid only in phase 2)
    pub flipslice_classidx: u16,
    pub flipslice_sym: u8,
    pub flipslice_rep: u32,
    pub corner_classidx: u16,
    pub corner_sym: u8,
    pub corner_rep: u16,
}

impl CoordCube {
    // Create a new 'CoordCube' in the solved state
    pub fn new() -> Self {
        Self {
            twist: SOLVED,
            flip: SOLVED as u16,
            slice_sorted: SOLVED as u16,
            u_edges: 1656, // Index of the solved u_edges
            d_edges: SOLVED as u16,
            corners: SOLVED,
            ud_edges: SOLVED as isize,
            flipslice_classidx: SOLVED as u16,
            flipslice_sym: SOLVED as u8,
            flipslice_rep: SOLVED as u32,
            corner_classidx: SOLVED as u16,
            corner_sym: SOLVED as u8,
            corner_rep: SOLVED as u16,
        }
    }

    // Creates a new 'CoordCube' from CubieCube
    pub fn from_cubie_cube(cc: &cb::CubieCube) -> Self {
        let twist: usize = cc.get_twist();
        let flip: u16 = cc.get_flip();
        let slice_sorted: u16 = cc.get_slice_sorted();
        let corners: usize = cc.get_corners();
        let ud_edges: isize = if slice_sorted < N_PERM_4 as u16 {
            cc.get_ud_edges() as isize
        } else {
            -1 // Invalid for phase 1
        };
        let u_edges: u16 = cc.get_u_edges();
        let d_edges: u16 = cc.get_d_edges();
        // Symmetry reduced flipslice coordinate used in phase 1
        let flipslice_classidx =
            FLIPSLICE_CLASSIDX[N_FLIP * (slice_sorted as usize / N_PERM_4) + flip as usize];
        let flipslice_sym =
            FLIPSLICE_SYM[N_FLIP * (slice_sorted as usize / N_PERM_4) + flip as usize];
        let flipslice_rep = FLIPSLICE_REP[flipslice_classidx as usize];
        // Symmetry reduced corner permutatoin coordinate used in phase 2
        let corner_classidx = CORNER_CLASSIDX[corners];
        let corner_sym = CORNER_SYM[corners];
        let corner_rep = CORNER_REP[corner_classidx as usize];

        Self {
            twist,
            flip,
            slice_sorted,
            corners,
            ud_edges,
            u_edges,
            d_edges,
            flipslice_classidx,
            flipslice_sym,
            flipslice_rep,
            corner_classidx,
            corner_sym,
            corner_rep,
        }
    }

    /// Updates phase 1 coordinates when move is applied
    pub fn phase1_move(&mut self, m: usize) {
        // Apply moves using the tables
        self.twist = TWIST_MOVE[N_MOVE * self.twist + m] as usize;
        self.flip = FLIP_MOVE[N_MOVE * self.flip as usize + m];
        self.slice_sorted = SLICE_SORTED_MOVE[N_MOVE * self.slice_sorted as usize + m];
        self.u_edges = U_EDGES_MOVE[N_MOVE * self.u_edges as usize + m];
        self.d_edges = D_EDGES_MOVE[N_MOVE * self.d_edges as usize + m];
        self.corners = CORNERS_MOVE[N_MOVE * self.corners + m] as usize;

        self.flipslice_classidx = FLIPSLICE_CLASSIDX
            [N_FLIP * (self.slice_sorted as usize / N_PERM_4) + self.flip as usize];
        self.flipslice_sym =
            FLIPSLICE_SYM[N_FLIP * (self.slice_sorted as usize / N_PERM_4) + self.flip as usize];
        self.flipslice_rep = FLIPSLICE_REP[self.flipslice_classidx as usize];

        self.corner_classidx = CORNER_CLASSIDX[self.corners];
        self.corner_sym = CORNER_SYM[self.corners];
        self.corner_rep = CORNER_REP[self.corner_classidx as usize];
    }

    /// Updates phase 2 coordinates when move is applied
    pub fn phase2_move(&mut self, m: usize) {
        // Apply moves using tables
        self.slice_sorted = SLICE_SORTED_MOVE[N_MOVE * self.slice_sorted as usize + m];
        self.corners = CORNERS_MOVE[N_MOVE * self.corners + m] as usize;
        self.ud_edges = UD_EDGES_MOVE[N_MOVE * self.ud_edges as usize + m] as isize;
    }

    /// Compute the distance to the cube subgroup H where flip=slice=twist=0
    pub fn get_depth_phase1(&self) -> usize {
        // Extract coordinates
        let mut slice = self.slice_sorted / N_PERM_4 as u16;
        let mut flip = self.flip;
        let mut twist = self.twist;
        let flipslice = N_FLIP * slice as usize + flip as usize;

        let classidx = FLIPSLICE_CLASSIDX[flipslice];
        let sym = FLIPSLICE_SYM[flipslice];
        let mut depth_mod3 = pr::get_flipslice_twist_depth3(
            N_TWIST * classidx as usize + TWIST_CONJ[(twist << 4) + sym as usize] as usize,
        );

        let mut depth = 0;
        while flip != SOLVED as u16 || slice != SOLVED as u16 || twist != SOLVED {
            if depth_mod3 == 0 {
                depth_mod3 = 3;
            }
            for m in 0..N_MOVE {
                let twist1 = TWIST_MOVE[N_MOVE * twist + m] as usize;
                let flip1 = FLIP_MOVE[N_MOVE * flip as usize + m];
                let slice1 =
                    SLICE_SORTED_MOVE[N_MOVE * slice as usize * N_PERM_4 + m] / N_PERM_4 as u16;
                let flipslice1 = N_FLIP * slice1 as usize + flip1 as usize;
                let classidx1 = FLIPSLICE_CLASSIDX[flipslice1];
                let sym = FLIPSLICE_SYM[flipslice1];

                if pr::get_flipslice_twist_depth3(
                    N_TWIST * classidx1 as usize
                        + TWIST_CONJ[(twist1 << 4) + sym as usize] as usize,
                ) == depth_mod3 - 1
                {
                    depth += 1;
                    twist = twist1;
                    flip = flip1;
                    slice = slice1;
                    depth_mod3 -= 1;
                    break;
                }
            }
        }
        depth
    }

    /// Get distance to subgroup where only the UD-slice edges may be permuted in their slice (only 24/2 = 12 possible ways
    /// due to overall even parity). This is a lower bound for the number of moves to solve phase 2.
    pub fn get_depth_phase2(&self, mut corners: usize, mut ud_edges: usize) -> usize {
        // Load tables
        let classidx = CORNER_CLASSIDX[corners];
        let sym = CORNER_SYM[corners];
        let mut depth_mod3 = pr::get_corners_ud_edges_depth3(
            N_UD_EDGES * classidx as usize + UD_EDGES_CONJ[(ud_edges << 4) + sym as usize] as usize,
        );
        // unfilled entry, depth >= 11
        if depth_mod3 == 3 {
            return 11;
        }
        let mut depth = 0;
        while corners != SOLVED || ud_edges != SOLVED {
            if depth_mod3 == 0 {
                depth_mod3 = 3;
            }
            // Only iterate phase 2 moves
            for &m in &[
                enums::Move::U1,
                enums::Move::U2,
                enums::Move::U3,
                enums::Move::R2,
                enums::Move::F2,
                enums::Move::D1,
                enums::Move::D2,
                enums::Move::D3,
                enums::Move::L2,
                enums::Move::B2,
            ] {
                let corners1 = CORNERS_MOVE[N_MOVE * corners + m as usize] as usize;
                let ud_edges1 = UD_EDGES_MOVE[N_MOVE * ud_edges + m as usize] as usize;
                let classidx1 = CORNER_CLASSIDX[corners1];
                let sym = CORNER_SYM[corners1];

                if pr::get_corners_ud_edges_depth3(
                    N_UD_EDGES * classidx1 as usize
                        + UD_EDGES_CONJ[(ud_edges1 << 4) + sym as usize] as usize,
                ) == depth_mod3 - 1
                {
                    depth += 1;
                    corners = corners1;
                    ud_edges = ud_edges1;
                    depth_mod3 -= 1;
                    break;
                }
            }
        }
        depth
    }
}

impl Default for CoordCube {
    fn default() -> Self {
        Self::new()
    }
}

// Lazy static to load phase2_edgemerge table
pub static U_EDGES_PLUS_D_EDGES_TO_UD_EDGES: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "phase2_edgemerge";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open the file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare a buffer fo N_U_EDGES_PHASE2 * N_PERM_4 entries
    let size = N_U_EDGES_PHASE2 * N_PERM_4;
    let mut buffer = vec![0u16; size];

    // Beacause each entry is a 2-byte 'H' (unsigned short),
    // we'll read them as little-endian u16 values.
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }
    buffer
});
