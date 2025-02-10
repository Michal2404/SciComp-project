use crate::rubiks_two_phase::defs;

// Symmetry related functions- Symmetry considerations increase the performance of the solver
// Here we are just loading precomputed tables from Python's Code by Herbert Kociemba.
use super::cubie::{self as cb, MOVE_CUBE};
use super::defs::{
    N_CORNERS, N_CORNERS_CLASS, N_FLIP, N_FLIPSLICE_CLASS, N_MOVE, N_SLICE, N_SYM, N_SYM_D4H,
    N_TWIST, N_UD_EDGES,
};
use super::enums::{BasicSymmetry as BS, Corner as Co, Edge as Ed};

use once_cell::sync::Lazy;
use std::fs::File;
use std::io::Read;

// Permutation and orientations of the basic symmetries

// 120° clockwise rotation around the long diagonal URF-DBL
const CP_ROT_URF3: [Co; 8] = [
    Co::URF,
    Co::DFR,
    Co::DLF,
    Co::UFL,
    Co::UBR,
    Co::DRB,
    Co::DBL,
    Co::ULB,
];
const CO_ROT_URF3: [u8; 8] = [1, 2, 1, 2, 2, 1, 2, 1];
const EP_ROT_URF3: [Ed; 12] = [
    Ed::UF,
    Ed::FR,
    Ed::DF,
    Ed::FL,
    Ed::UB,
    Ed::BR,
    Ed::DB,
    Ed::BL,
    Ed::UR,
    Ed::DR,
    Ed::DL,
    Ed::UL,
];
const EO_ROT_URF3: [u8; 12] = [1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1];

// 180° rotation around the axis through F and B centers
const CP_ROT_F2: [Co; 8] = [
    Co::DLF,
    Co::DFR,
    Co::DRB,
    Co::DBL,
    Co::UFL,
    Co::URF,
    Co::UBR,
    Co::ULB,
];
const CO_ROT_F2: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
const EP_ROT_F2: [Ed; 12] = [
    Ed::DL,
    Ed::DF,
    Ed::DR,
    Ed::DB,
    Ed::UL,
    Ed::UF,
    Ed::UR,
    Ed::UB,
    Ed::FL,
    Ed::FR,
    Ed::BR,
    Ed::BL,
];
const EO_ROT_F2: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// 90° clockwise rotation around the axis through the U and D centers
const CP_ROT_U4: [Co; 8] = [
    Co::UBR,
    Co::URF,
    Co::UFL,
    Co::ULB,
    Co::DRB,
    Co::DFR,
    Co::DLF,
    Co::DBL,
];
const CO_ROT_U4: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
const EP_ROT_U4: [Ed; 12] = [
    Ed::UB,
    Ed::UR,
    Ed::UF,
    Ed::UL,
    Ed::DB,
    Ed::DR,
    Ed::DF,
    Ed::DL,
    Ed::BR,
    Ed::FR,
    Ed::FL,
    Ed::BL,
];
const EO_ROT_U4: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1];

// reflection at the plane through the U, D, F, B centers
const CP_MIRR_LR2: [Co; 8] = [
    Co::UFL,
    Co::URF,
    Co::UBR,
    Co::ULB,
    Co::DLF,
    Co::DFR,
    Co::DRB,
    Co::DBL,
];
const CO_MIRR_LR2: [u8; 8] = [3, 3, 3, 3, 3, 3, 3, 3];
const EP_MIRR_LR2: [Ed; 12] = [
    Ed::UL,
    Ed::UF,
    Ed::UR,
    Ed::UB,
    Ed::DL,
    Ed::DF,
    Ed::DR,
    Ed::DB,
    Ed::FL,
    Ed::FR,
    Ed::BR,
    Ed::BL,
];
const EO_MIRR_LR2: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const BASIC_SYM_CUBE_COUNT: usize = 4;

lazy_static::lazy_static! {
    static ref BASIC_SYM_CUBE: Vec<cb::CubieCube> = {
        let mut cubes = vec![cb::CubieCube::new(None, None, None, None); BASIC_SYM_CUBE_COUNT];
        cubes[0] = cb::CubieCube::new(Some(CP_ROT_URF3), Some(CO_ROT_URF3), Some(EP_ROT_URF3), Some(EO_ROT_URF3));
        cubes[1] = cb::CubieCube::new(Some(CP_ROT_F2), Some(CO_ROT_F2), Some(EP_ROT_F2), Some(EO_ROT_F2));
        cubes[2] = cb::CubieCube::new(Some(CP_ROT_U4), Some(CO_ROT_U4), Some(EP_ROT_U4), Some(EO_ROT_U4));
        cubes[3] = cb::CubieCube::new(Some(CP_MIRR_LR2), Some(CO_MIRR_LR2), Some(EP_MIRR_LR2), Some(EO_MIRR_LR2));
        cubes
    };
}

// Fill SymCube list
// 48 cubes will represent 48 symmetries

pub static SYM_CUBE: Lazy<Vec<cb::CubieCube>> = Lazy::new(|| {
    let mut result = Vec::with_capacity(N_SYM);

    // Start with an identity cube
    let mut cc = cb::CubieCube::new(None, None, None, None);

    for _urf3 in 0..3 {
        for _f2 in 0..2 {
            for _u4 in 0..4 {
                for _lr2 in 0..2 {
                    // Push a copy of cc into the result
                    result.push(cb::CubieCube::new(
                        Some(cc.cp),
                        Some(cc.co),
                        Some(cc.ep),
                        Some(cc.eo),
                    ));

                    cc.multiply(&BASIC_SYM_CUBE[BS::MirrLR2 as usize]);
                }
                cc.multiply(&BASIC_SYM_CUBE[BS::RotU4 as usize]);
            }
            cc.multiply(&BASIC_SYM_CUBE[BS::RotF2 as usize]);
        }
        cc.multiply(&BASIC_SYM_CUBE[BS::RotURF3 as usize]);
    }

    // result should have 48 entries
    assert_eq!(result.len(), N_SYM, "Expected exactly 48 symmetries");
    result
});

// The 'inv_idx' table: inv_idx[j] = i means SYM_CUBE[i] is the inverse of SYM_CUBE[j].
pub static INV_IDX: Lazy<[u8; N_SYM]> = Lazy::new(|| {
    let mut inv = [0u8; N_SYM];

    for j in 0..N_SYM {
        for i in 0..N_SYM {
            // Clone symCube[j] (or create a new CubieCube from its fields).
            let mut cc = cb::CubieCube::new(
                Some(SYM_CUBE[j].cp),
                Some(SYM_CUBE[j].co),
                Some(SYM_CUBE[j].ep),
                Some(SYM_CUBE[j].eo),
            );
            // Multiply by symCube[i].
            cc.corner_multiply(&SYM_CUBE[i]);

            // Check if the corners URF, UFL, ULB are in their solved positions.
            if cc.cp[Co::URF as usize] == Co::URF
                && cc.cp[Co::UFL as usize] == Co::UFL
                && cc.cp[Co::ULB as usize] == Co::ULB
            {
                inv[j] = i as u8;
                break;
            }
        }
    }

    inv
});

// Generate the group table for the 48 cube symmetries
pub static MULT_SYM: Lazy<[u8; N_SYM * N_SYM]> = Lazy::new(|| {
    let mut table = [0u8; N_SYM * N_SYM];

    // For each pair (i, j) compute SYM_CUBE[i] * SYM_CUBE[j],
    // then find k such that it equals SYM_CUBE[k].
    for j in 0..N_SYM {
        for i in 0..N_SYM {
            let mut cc = SYM_CUBE[i];
            cc.multiply(&SYM_CUBE[j]);

            // Find k such that cc == SYM_CUBE[k]
            for k in 0..N_SYM {
                if cc == SYM_CUBE[k] {
                    table[i * N_SYM + j] = k as u8;
                    break;
                }
            }
        }
    }

    table
});

/// Generate the table for the conjugation of a move m by a symmetry s. CONJ_MOVE[N_MOVE*s + m] = s*m*s^-1
pub static CONJ_MOVE: Lazy<[u16; N_MOVE * N_SYM]> = Lazy::new(|| {
    let mut table = [0u16; N_MOVE * N_SYM];

    for s in 0..N_SYM {
        for m in 0..N_MOVE {
            let mut ss = SYM_CUBE[s];
            ss.multiply(&MOVE_CUBE[m]);
            ss.multiply(&SYM_CUBE[INV_IDX[s] as usize]);

            for m2 in 0..N_MOVE {
                if ss == MOVE_CUBE[m2] {
                    table[N_MOVE * s + m] = m2 as u16;
                    break;
                }
            }
        }
    }
    table
});

/// Generate the phase 1 table for the conjugation of the twist t by a symmetry s. twist_conj[t, s] = s*t*s^-1
pub static TWIST_CONJ: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path

    let fname = "conj_twist";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Cannot open conj_twist file at {:?}", path));

    // We have N_TWIST * N_SYM_D4H entries, each 2 bytes (u16).
    let size = N_TWIST * N_SYM_D4H;
    let mut buffer = vec![0u16; size];

    // Read them from the file in little-endian order.
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }

    buffer
});

// load the phase 2 table for the conjugation of the URtoDB by a symmetrie
pub static UD_EDGES_CONJ: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "conj_ud_edges";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Cannot open conj_twist file at {:?}", path));

    // We have N_UD_EDGES * N_SYM_D4H entries, each 2 bytes (u16).
    let size = N_UD_EDGES * N_SYM_D4H;
    let mut buffer = vec![0u16; size];

    // Read them from the file in little-endian order.
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }

    buffer
});

// Load the tables to handle the symmetry reduced flip-slice coordinate in phase 1
pub static FLIPSLICE_CLASSIDX: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path

    let fname = "fs_classidx";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open the file with detailed error handling
    let mut file = File::open(&path).unwrap_or_else(|err| {
        eprintln!(
            "Error: Cannot open file {:?} - {}. Please ensure the file exists and is accessible.",
            path, err
        );
        std::process::exit(1); // Graceful exit
    });

    // We have N_FLIP * N_SLICE entries, each 2 bytes (u16).
    let size = N_FLIP * N_SLICE;
    let mut buffer = vec![0u16; size];

    // Read them from the file in little-endian order.
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }

    buffer
});

pub static FLIPSLICE_SYM: Lazy<Vec<u8>> = Lazy::new(|| {
    // Build the file path
    let fname = "fs_sym";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Cannot open conj_twist file at {:?}", path));

    // We have N_FLIP * N_SLICE entries, each 2 bytes (u16).
    let size = N_FLIP * N_SLICE;
    let mut buffer = vec![0u8; size];

    // Read them from the file in little-endian order.
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut byte = [0u8; 1];
        file.read_exact(&mut byte)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = byte[0];
    }

    buffer
});

pub static FLIPSLICE_REP: Lazy<Vec<u32>> = Lazy::new(|| {
    // Build the file path
    let fname = "fs_rep";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Cannot open conj_twist file at {:?}", path));

    // We have N_FLIP * N_SLICE entries, each 2 bytes (u16).
    let size = N_FLIPSLICE_CLASS;
    let mut buffer = vec![0u32; size];

    // Read them from the file in little-endian order.
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 4];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u32::from_le_bytes(bytes);
    }

    buffer
});

// Load the tables to handle the symmetry reduced corner permutation coordinate in phase 2
pub static CORNER_CLASSIDX: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "co_classidx";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Cannot open conj_twist file at {:?}", path));

    // We have N_FLIP * N_SLICE entries, each 2 bytes (u16).
    let size = N_CORNERS;
    let mut buffer = vec![0u16; size];

    // Read them from the file in little-endian order.
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }

    buffer
});

pub static CORNER_SYM: Lazy<Vec<u8>> = Lazy::new(|| {
    // Build the file path
    let fname = "co_sym";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Cannot open conj_twist file at {:?}", path));

    // We have N_FLIP * N_SLICE entries, each 2 bytes (u16).
    let size = N_CORNERS;
    let mut buffer = vec![0u8; size];

    // Read them from the file in little-endian order.
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut byte = [0u8; 1];
        file.read_exact(&mut byte)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = byte[0];
    }

    buffer
});

pub static CORNER_REP: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "co_rep";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Cannot open conj_twist file at {:?}", path));

    // We have N_FLIP * N_SLICE entries, each 2 bytes (u16).
    let size = N_CORNERS_CLASS;
    let mut buffer = vec![0u16; size];

    // Read them from the file in little-endian order.
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }

    buffer
});
