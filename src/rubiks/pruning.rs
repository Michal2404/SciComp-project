// The pruning tables cut the search tree during search
// The pruning values are stored module 3 which saves a lot of memory.

use crate::rubiks::defs::N_CORNERS_CLASS;
use crate::rubiks::defs::N_FLIPSLICE_CLASS;
use crate::rubiks::defs::N_PERM_4;

use super::defs;
use super::defs::{FOLDER, N_CORNERS, N_TWIST, N_UD_EDGES};
use byteorder::{LittleEndian, ReadBytesExt};
use once_cell::sync::Lazy;
use std::fs::File;
use std::path::Path;

use std::io::Read;

/////////////////////////////////////////////////////////////
//Functions to extract or set values in the pruning tables //
/////////////////////////////////////////////////////////////

/// returns *exactly* number of moves % 3 to solve phase 1
pub fn get_flipslice_twist_depth3(ix: usize) -> u32 {
    let y = FLIPSLICE_TWIST_DEPTH3[ix / 16];
    ((y >> ((ix % 16) * 2)) & 3) as u32
}

/// Extract the depth3 value for corners_ud_edges
pub fn get_corners_ud_edges_depth3(ix: usize) -> u32 {
    let y = CORNERS_UD_EDGES_DEPTH3[ix / 16];
    ((y >> ((ix % 16) * 2)) & 3) as u32
}

//////////////////////////////////////
// Functions to load pruning tables //
//////////////////////////////////////

pub static FLIPSLICE_TWIST_DEPTH3: Lazy<Vec<u32>> = Lazy::new(|| {
    let total = N_FLIPSLICE_CLASS * N_TWIST;
    let count = total / 16 + 1;

    // Build the file path
    let fname = "phase1_prun";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare buffer
    let mut buffer = vec![0u32; count];

    // Read each 32-bit (4 byte) value in little-endian
    for i in 0..count {
        let mut bytes = [0u8; 4];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        buffer[i] = u32::from_le_bytes(bytes);
    }
    buffer
});

pub static CORNERS_UD_EDGES_DEPTH3: Lazy<Vec<u32>> = Lazy::new(|| {
    let total = N_CORNERS_CLASS * N_UD_EDGES;
    let count = total / 16;

    // Build the file path
    let fname = "phase2_prun";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare buffer
    let mut buffer = vec![0u32; count];

    // Read each 32-bit (4 byte) value in little-endian
    for i in 0..count {
        let mut bytes = [0u8; 4];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        buffer[i] = u32::from_le_bytes(bytes);
    }
    buffer
});

pub static CORNSLICE_DEPTH: Lazy<Vec<i8>> = Lazy::new(|| {
    // Build the file path
    let fname = "phase2_cornsliceprun";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare buffer
    let size = N_CORNERS * N_PERM_4;
    let mut buffer = vec![0i8; size];

    // Read each 32-bit (4 byte) value in little-endian
    for i in 0..size {
        let mut byte = [0u8; 1];
        file.read_exact(&mut byte)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        buffer[i] = byte[0] as i8;
    }
    buffer
});

// This is now a lazily computed static.
pub static DISTANCE: Lazy<[i8; 60]> = Lazy::new(|| {
    let mut d = [0i8; 60];
    for i in 0..20 {
        for j in 0..3 {
            let index = 3 * i + j;
            d[index] = ((i / 3) * 3 + j) as i8;
            if i % 3 == 2 && j == 0 {
                d[index] += 3;
            } else if i % 3 == 0 && j == 2 {
                d[index] -= 3;
            }
        }
    }
    d
});
