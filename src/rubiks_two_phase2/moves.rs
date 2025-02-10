use super::defs;
/// Movetables describe the transformation of the coordinates by cube moves
/// We don't generate the move tables here, but load the pre-generated tables from the
/// Herbert Kociemba's Python Script.
use super::defs::{N_CORNERS, N_FLIP, N_MOVE, N_SLICE_SORTED, N_TWIST, N_UD_EDGES};
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::Read;

/// Saves the transformations of the corner orientation coordinates. (2187 x 18) - phase 1
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
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }
    buffer
});

/// Saves the transformations of the edge orientation coordinates. (2048 x 18) - phase 1
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
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }
    buffer
});

/// Saves the transofrmations to sorted UD-edges coordinates. (11880 x 18) - phase 1
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
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }
    buffer
});

/// Saves transormations of the U Edge permutation coordinates (11880 x 18) phase 1
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
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }
    buffer
});

// Saves transformation of the D Edge permutation coordinates (11880 x 18) phase 2
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
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }
    buffer
});

// Saves the transformations of the UD Edges permutation coordiantes (40320 x 18) phase 2
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
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }
    buffer
});

// Saves the transformations of the corner permutation coordinates (40320 x 18) phase 2
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
    for (i, item) in buffer.iter_mut().enumerate().take(size) {
        let mut bytes = [0u8; 2];
        file.read_exact(&mut bytes)
            .unwrap_or_else(|_| panic!("Error reading entry {} from {:?}", i, path));
        *item = u16::from_le_bytes(bytes);
    }
    buffer
});
