// The pruning tables cut the search tree during search
// The pruning values are stored module 3 which saves a lot of memory.

use super::cubie as cb;
use super::defs;
use super::defs::{FOLDER, N_CORNERS, N_FLIP, N_MOVE, N_SLICE_SORTED, N_TWIST, N_UD_EDGES};
use super::enums;
use super::moves as mv;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

pub fn generate_phase1_prun_table() -> io::Result<()> {
    let fname = "phase1_prun";
    let path = Path::new(FOLDER).join(fname);
    // load it
    println!("loading {} table...", fname);
    let mut file = File::open(path).expect("Unable to open file");

    let mut reader = BufReader::new(file);

    let total = defs::N_FLIPSLICE_CLASS * defs::N_TWIST;
    let array_size = total / 16 + 1;

    let mut flipslice_twist_depth3 = Vec::with_capacity(array_size);

    for _ in 0..array_size {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        flipslice_twist_depth3.push(u32::from_le_bytes(buffer));
    }

    Ok(())
}
