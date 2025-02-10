// Constants and definitions for the cube

use std::path::PathBuf;

use super::cubie::CubieCube;
use super::enums::{Color as Cl, Corner as Co, Edge as Ed, Facelet as Fc};

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

// Basic six cube moves described by permutations and changes in orientation on the Cubie Level
// Up-move
pub const CP_U: [Co; 8] = [
    Co::UBR,
    Co::URF,
    Co::UFL,
    Co::ULB,
    Co::DFR,
    Co::DLF,
    Co::DBL,
    Co::DRB,
];
pub const CO_U: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
pub const EP_U: [Ed; 12] = [
    Ed::UB,
    Ed::UR,
    Ed::UF,
    Ed::UL,
    Ed::DR,
    Ed::DF,
    Ed::DL,
    Ed::DB,
    Ed::FR,
    Ed::FL,
    Ed::BL,
    Ed::BR,
];
pub const EO_U: [u8; 12] = [0; 12];
pub const TURN_U: CubieCube = CubieCube {
    cp: CP_U,
    co: CO_U,
    ep: EP_U,
    eo: EO_U,
};

// Right-move
pub const CP_R: [Co; 8] = [
    Co::DFR,
    Co::UFL,
    Co::ULB,
    Co::URF,
    Co::DRB,
    Co::DLF,
    Co::DBL,
    Co::UBR,
];
pub const CO_R: [u8; 8] = [2, 0, 0, 1, 1, 0, 0, 2];
pub const EP_R: [Ed; 12] = [
    Ed::FR,
    Ed::UF,
    Ed::UL,
    Ed::UB,
    Ed::BR,
    Ed::DF,
    Ed::DL,
    Ed::DB,
    Ed::DR,
    Ed::FL,
    Ed::BL,
    Ed::UR,
];
pub const EO_R: [u8; 12] = [0; 12];
pub const TURN_R: CubieCube = CubieCube {
    cp: CP_R,
    co: CO_R,
    ep: EP_R,
    eo: EO_R,
};

// Front-move
pub const CP_F: [Co; 8] = [
    Co::UFL,
    Co::DLF,
    Co::ULB,
    Co::UBR,
    Co::URF,
    Co::DFR,
    Co::DBL,
    Co::DRB,
];
pub const CO_F: [u8; 8] = [1, 2, 0, 0, 2, 1, 0, 0];
pub const EP_F: [Ed; 12] = [
    Ed::UR,
    Ed::FL,
    Ed::UL,
    Ed::UB,
    Ed::DR,
    Ed::FR,
    Ed::DL,
    Ed::DB,
    Ed::UF,
    Ed::DF,
    Ed::BL,
    Ed::BR,
];
pub const EO_F: [u8; 12] = [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0];
pub const TURN_F: CubieCube = CubieCube {
    cp: CP_F,
    co: CO_F,
    ep: EP_F,
    eo: EO_F,
};

// Down-move
pub const CP_D: [Co; 8] = [
    Co::URF,
    Co::UFL,
    Co::ULB,
    Co::UBR,
    Co::DLF,
    Co::DBL,
    Co::DRB,
    Co::DFR,
];
pub const CO_D: [u8; 8] = [0; 8];
pub const EP_D: [Ed; 12] = [
    Ed::UR,
    Ed::UF,
    Ed::UL,
    Ed::UB,
    Ed::DF,
    Ed::DL,
    Ed::DB,
    Ed::DR,
    Ed::FR,
    Ed::FL,
    Ed::BL,
    Ed::BR,
];
pub const EO_D: [u8; 12] = [0; 12];
pub const TURN_D: CubieCube = CubieCube {
    cp: CP_D,
    co: CO_D,
    ep: EP_D,
    eo: EO_D,
};

// Left-move
pub const CP_L: [Co; 8] = [
    Co::URF,
    Co::ULB,
    Co::DBL,
    Co::UBR,
    Co::DFR,
    Co::UFL,
    Co::DLF,
    Co::DRB,
];
pub const CO_L: [u8; 8] = [0, 1, 2, 0, 0, 2, 1, 0];
pub const EP_L: [Ed; 12] = [
    Ed::UR,
    Ed::UF,
    Ed::BL,
    Ed::UB,
    Ed::DR,
    Ed::DF,
    Ed::FL,
    Ed::DB,
    Ed::FR,
    Ed::UL,
    Ed::DL,
    Ed::BR,
];
pub const EO_L: [u8; 12] = [0; 12];
pub const TURN_L: CubieCube = CubieCube {
    cp: CP_L,
    co: CO_L,
    ep: EP_L,
    eo: EO_L,
};

// Back-move
pub const CP_B: [Co; 8] = [
    Co::URF,
    Co::UFL,
    Co::UBR,
    Co::DRB,
    Co::DFR,
    Co::DLF,
    Co::ULB,
    Co::DBL,
];
pub const CO_B: [u8; 8] = [0, 0, 1, 2, 0, 0, 2, 1];
pub const EP_B: [Ed; 12] = [
    Ed::UR,
    Ed::UF,
    Ed::UL,
    Ed::BR,
    Ed::DR,
    Ed::DF,
    Ed::DL,
    Ed::BL,
    Ed::FR,
    Ed::FL,
    Ed::UB,
    Ed::DB,
];
pub const EO_B: [u8; 12] = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1];
pub const TURN_B: CubieCube = CubieCube {
    cp: CP_B,
    co: CO_B,
    ep: EP_B,
    eo: EO_B,
};

pub const CUBE_OK: bool = true;

// Constants
pub const N_PERM_4: usize = 24; // Number of possible permutations of 4 cubies (UD-Slice Phase 2)
pub const N_CHOOSE_8_4: usize = 70; // Possible permutations of 4 pieces on 8 slots (used in phase2_edgemerge table)
pub const N_MOVE: usize = 18; // Number of possible HTM moves

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
pub const FOLDER: &str = "rubiks_two_phase/twophase"; // Folder for generated tables

// Dynamically resolve the path  the rubiks/twophase folder
pub fn get_folder_path() -> PathBuf {
    let project_root = std::env::current_dir().expect("Failed to get current directory");
    project_root
        .join("src")
        .join("rubiks_two_phase")
        .join("twophase")
}

// Get the full path to a specific file in the rubiks/twophase folder
pub fn get_table_path(fname: &str) -> PathBuf {
    get_folder_path().join(fname)
}
