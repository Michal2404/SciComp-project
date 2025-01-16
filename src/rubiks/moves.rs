use crate::rubiks::defs::N_SLICE;

use super::defs;
/// Movetables describe the transformation of the coordinates by cube moves
/// We don't generate the move tables here, but load the pre-generated tables from the
/// Herbert Kociemba's Python Script.
use super::defs::{FOLDER, N_CORNERS, N_FLIP, N_MOVE, N_SLICE_SORTED, N_TWIST, N_UD_EDGES};
use byteorder::{LittleEndian, ReadBytesExt};
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub static TWIST_MOVE: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "move_twist";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open the file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare a buffer fo N_TWIST * N_MOVE entries
    let size = N_TWIST * N_MOVE;
    let mut buffer = vec![0u16; size];

    // Beacause each entry is a 2-byte 'H' (unsigned short),
    // we'll read them as little-endian u16 values.
    for i in 0..size {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        buffer[i] = u16::from_le_bytes(bytes);
    }
    buffer
});

pub static FLIP_MOVE: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "move_flip";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open the file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare a buffer for N_FLIP * N_MOVE entries
    let size: usize = N_FLIP * N_MOVE;
    let mut buffer = vec![0u16; size];

    // Beacause each entry is a 2-byte 'H' (unsigned short),
    // we'll read them as little-endian u16 values.
    for i in 0..size {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        buffer[i] = u16::from_le_bytes(bytes);
    }
    buffer
});

pub static SLICE_SORTED_MOVE: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "move_slice_sorted";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open the file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare a buffer fo N_SLICE_SORTED * N_MOVE entries
    let size = N_SLICE_SORTED * N_MOVE;
    let mut buffer = vec![0u16; size];

    // Beacause each entry is a 2-byte 'H' (unsigned short),
    // we'll read them as little-endian u16 values.
    for i in 0..size {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        buffer[i] = u16::from_le_bytes(bytes);
    }
    buffer
});

pub static U_EDGES_MOVE: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "move_u_edges";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open the file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare a buffer fo N_SLICE_SORTED * N_MOVE entries
    let size = N_SLICE_SORTED * N_MOVE;
    let mut buffer = vec![0u16; size];

    // Beacause each entry is a 2-byte 'H' (unsigned short),
    // we'll read them as little-endian u16 values.
    for i in 0..size {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        buffer[i] = u16::from_le_bytes(bytes);
    }
    buffer
});

pub static D_EDGES_MOVE: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "move_d_edges";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open the file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare a buffer fo N_SLICE_SORTED * N_MOVE entries
    let size = N_SLICE_SORTED * N_MOVE;
    let mut buffer = vec![0u16; size];

    // Beacause each entry is a 2-byte 'H' (unsigned short),
    // we'll read them as little-endian u16 values.
    for i in 0..size {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        buffer[i] = u16::from_le_bytes(bytes);
    }
    buffer
});

pub static UD_EDGES_MOVE: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "move_ud_edges";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open the file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare a buffer fo N_UD_EDGES * N_MOVE entries
    let size = N_UD_EDGES * N_MOVE;
    let mut buffer = vec![0u16; size];

    // Beacause each entry is a 2-byte 'H' (unsigned short),
    // we'll read them as little-endian u16 values.
    for i in 0..size {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        buffer[i] = u16::from_le_bytes(bytes);
    }
    buffer
});

pub static CORNERS_MOVE: Lazy<Vec<u16>> = Lazy::new(|| {
    // Build the file path
    let fname = "move_corners";
    let path = defs::get_table_path(fname);

    if !path.exists() {
        eprintln!("Error: Table file {:?} does not exist.", path);
        std::process::exit(1);
    }

    println!("Loading {} table", fname);

    // Open the file
    let mut file = File::open(&path).unwrap_or_else(|_| panic!("Cannot open {:?}", path));

    // Prepare a buffer fo N_CORNERS * N_MOVE entries
    let size = N_CORNERS * N_MOVE;
    let mut buffer = vec![0u16; size];

    // Beacause each entry is a 2-byte 'H' (unsigned short),
    // we'll read them as little-endian u16 values.
    for i in 0..size {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        buffer[i] = u16::from_le_bytes(bytes);
    }
    buffer
});
