// Constants and definitions for the cube

use super::enums::{Color as Cl, Facelet as Fc};

// Map the corner positions to facelet positions
pub const CORNER_FACELET: [[Fc; 3]; 8] = [
    [Fc::U9, Fc::R1, Fc::F3],
    [Fc::U7, Fc::F1, Fc::L3],
    [Fc::U1, Fc::L1, Fc::B3],
    [Fc::U3, Fc::B1, Fc::R3],
    [Fc::D3, Fc::F9, Fc::R7],
    [Fc::D1, Fc::L9, Fc::F7],
    [Fc::D7, Fc::B9, Fc::L7],
    [Fc::D9, Fc::R9, Fc::B7],
];

// Map the edge positions to facelet positions
pub const EDGE_FACELET: [[Fc; 2]; 12] = [
    [Fc::U6, Fc::R2],
    [Fc::U8, Fc::F2],
    [Fc::U4, Fc::L2],
    [Fc::U2, Fc::B2],
    [Fc::D6, Fc::R8],
    [Fc::D2, Fc::F8],
    [Fc::D4, Fc::L8],
    [Fc::D8, Fc::B8],
    [Fc::F6, Fc::R4],
    [Fc::F4, Fc::L6],
    [Fc::B6, Fc::L4],
    [Fc::B4, Fc::R6],
];

// Map the corner positions to facelet colors
pub const CORNER_COLOR: [[Cl; 3]; 8] = [
    [Cl::U, Cl::R, Cl::F],
    [Cl::U, Cl::F, Cl::L],
    [Cl::U, Cl::L, Cl::B],
    [Cl::U, Cl::B, Cl::R],
    [Cl::D, Cl::F, Cl::R],
    [Cl::D, Cl::L, Cl::F],
    [Cl::D, Cl::B, Cl::L],
    [Cl::D, Cl::R, Cl::B],
];

// Map the edge positions to facelet colors
pub const EDGE_COLOR: [[Cl; 2]; 12] = [
    [Cl::U, Cl::R],
    [Cl::U, Cl::F],
    [Cl::U, Cl::L],
    [Cl::U, Cl::B],
    [Cl::D, Cl::R],
    [Cl::D, Cl::F],
    [Cl::D, Cl::L],
    [Cl::D, Cl::B],
    [Cl::F, Cl::R],
    [Cl::F, Cl::L],
    [Cl::B, Cl::L],
    [Cl::B, Cl::R],
];

// Constants
pub const N_PERM_4: usize = 24;
pub const N_CHOOSE_8_4: usize = 70;
pub const N_MOVE: usize = 18;

pub const N_TWIST: usize = 2187; // 3^7 possible corner orientations in phase 1
pub const N_FLIP: usize = 2048; // 2^11 possible edge orientations in phase 1
pub const N_SLICE_SORTED: usize = 11880; // 12*11*10*9 positions of edges in phase 1
pub const N_SLICE: usize = N_SLICE_SORTED / N_PERM_4; // Ignoring permutation of FR, FL, BL, BR in phase 1
pub const N_FLIPSLICE_CLASS: usize = 64430; // Combined flip+slice equivalence classes for symmetry group D4h

pub const N_U_EDGES_PHASE2: usize = 1680; // UR, UF, UL, UB positions in phase 2
pub const N_D_EDGES_PHASE2: usize = 1680; // number of different positions of the edges DR, DF, DL and DB in phase 2
pub const N_CORNERS: usize = 40320; // 8! corner permutations in phase 2
pub const N_CORNERS_CLASS: usize = 2768; // Equivalence classes for symmetry group D4h
pub const N_UD_EDGES: usize = 40320; // 8! U-face and D-face edge permutations in phase 2

pub const N_SYM: usize = 48; // Cube symmetries of group Oh
pub const N_SYM_D4H: usize = 16; // Subgroup D4h symmetries
pub const FOLDER: &str = "twophase"; // Folder for generated tables
