/// Movetables describe the transformation of the coordinates by cube moves
use super::cubie as cb;
use super::defs::{FOLDER, N_CORNERS, N_FLIP, N_MOVE, N_SLICE_SORTED, N_TWIST, N_UD_EDGES};
use super::enums;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::path::Path;

pub fn generate_move_table() {
    /////////////////////////////////////////////
    // Move table for the twists of the corners//
    /////////////////////////////////////////////
    let fname = "move_twist";
    let path = Path::new(FOLDER).join(fname);

    // If file doesn't exist, create it
    if !path.exists() {
        println!("creating {} table...", fname);
        let mut twist_move = vec![0u16; N_TWIST * N_MOVE];

        let mut a = cb::CubieCube::new(None, None, None, None);

        for i in 0..N_TWIST {
            a.set_twist(i as u16);

            for &c1 in enums::Color::iter() {
                for k1 in 0..3 {
                    // three moves for each face
                    // Apply the basic move (corner_multiply)
                    a.corner_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);

                    // Store the twist after the move
                    twist_move[N_MOVE * i + 3 * c1 as usize + k1] = a.get_twist() as u16;
                }
                // 4th move restores the face
                a.corner_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);
            }
        }

        // Write to file
        let mut file = File::create(path).expect("Unable to create file");
        for value in twist_move {
            file.write_u16::<LittleEndian>(value)
                .expect("Unable to write data");
        }
    } else {
        // If the file exists, load it
        println!("loading {} table...", fname);
        let mut file = File::open(path).expect("Unable to open file");
        let mut twist_move = vec![0u16; N_TWIST * N_MOVE];

        // Read from the file
        for i in 0..(N_TWIST * N_MOVE) {
            twist_move[i] = file
                .read_u16::<LittleEndian>()
                .expect("Unable to read data");
        }
    }

    /////////////////////////////////////////
    // Move table for the flip of the edges//
    /////////////////////////////////////////
    let fname = "move_flip";
    let path = Path::new(FOLDER).join(fname);

    if !path.exists() {
        println!("creating {} table...", fname);
        let mut flip_move = vec![0u16; N_FLIP * N_MOVE];
        let mut a = cb::CubieCube::new(None, None, None, None);
        for i in 0..N_FLIP {
            a.set_flip(i as u16);
            for &c1 in enums::Color::iter() {
                for k1 in 0..3 {
                    // three moves for each face
                    // Apply the basic move (corner_multiply)
                    a.edge_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);

                    // Store the twist after the move
                    flip_move[N_MOVE * i + 3 * c1 as usize + k1] = a.get_flip() as u16;
                }
                // 4th move restores the face
                a.edge_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);
            }
        }
        // Write to file
        let mut file = File::create(path).expect("Unable to create file");
        for value in flip_move {
            file.write_u16::<LittleEndian>(value)
                .expect("Unable to write data");
        }
    } else {
        // If the file exists, load it
        println!("loading {} table...", fname);
        let mut file = File::open(path).expect("Unable to open file");
        let mut flip_move = vec![0u16; N_FLIP * N_MOVE];

        // Read from the file
        for i in 0..(N_FLIP * N_MOVE) {
            flip_move[i] = file
                .read_u16::<LittleEndian>()
                .expect("Unable to read data");
        }
    }

    ///////////////////////////////////////////
    // Move table for the four UD-slice edges//
    ///////////////////////////////////////////

    // The slice sorted coordinate describes 12!/8! = 11880 possible positions of the FR, FL, BL and BR edges.
    // Though for phase 1 only the "unsorted" slice coordinate with Binomial(12,4) = 495 positions is relevant, using the
    // slice_sorted coordinate gives us the permutation of the FR, FL, BL and BR edges at the beginning of phase 2 for free.
    // 0 <= slice_sorted <= 11880 in phase 1, 0 <= slice_sorted <= 24 in phase 2, slice_sorted=0 for solved cube.
    let fname = "move_slice_sorted";
    let path = Path::new(FOLDER).join(fname);

    // If file doesn't exist, create it
    if !path.exists() {
        println!("creating {} table...", fname);
        let mut slice_sorted_move = vec![0u16; N_SLICE_SORTED * N_MOVE];

        let mut a = cb::CubieCube::new(None, None, None, None);

        for i in 0..N_SLICE_SORTED {
            if i % 200 == 0 {
                print!(".");
            }
            a.set_slice_sorted(i);

            for &c1 in enums::Color::iter() {
                for k1 in 0..3 {
                    // three moves for each face
                    // Apply the basic move (corner_multiply)
                    a.edge_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);

                    // Store the twist after the move
                    slice_sorted_move[N_MOVE * i + 3 * c1 as usize + k1] =
                        a.get_slice_sorted() as u16;
                }
                // 4th move restores the face
                a.edge_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);
            }
        }

        // Write to file
        let mut file = File::create(path).expect("Unable to create file");
        for value in slice_sorted_move {
            file.write_u16::<LittleEndian>(value)
                .expect("Unable to write data");
        }
    } else {
        // If the file exists, load it
        println!("loading {} table...", fname);
        let mut file = File::open(path).expect("Unable to open file");
        let mut slice_sorted_move = vec![0u16; N_SLICE_SORTED * N_MOVE];

        // Read from the file
        for i in 0..(N_SLICE_SORTED * N_MOVE) {
            slice_sorted_move[i] = file
                .read_u16::<LittleEndian>()
                .expect("Unable to read data");
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Move table for the u_edges coordinate for transition phase 1 -> phase 2//
    ////////////////////////////////////////////////////////////////////////////
    // The u_edges coordinate describes the 12!/8! = 11880 possible positions of the UR, UF, UL and UB edges.
    // It is needed at the end of phase 1 to set up the coordinates of phase 2
    // 0 <= u_edges <= 11880 in phase 1, 0 <= u_edges <= 1680 in phase 2, u_edges = 1656 for solved cube.
    let fname = "move_u_edges";
    let path = Path::new(FOLDER).join(fname);

    if !path.exists() {
        println!("creating {} table...", fname);
        let mut u_edges_move = vec![0u16; N_SLICE_SORTED * N_MOVE];
        let mut a = cb::CubieCube::new(None, None, None, None);
        for i in 0..N_SLICE_SORTED {
            if i % 200 == 0 {
                print!(".");
            }
            a.set_u_edges(i);
            for &c1 in enums::Color::iter() {
                for k1 in 0..3 {
                    // three moves for each face
                    // Apply the basic move (corner_multiply)
                    a.edge_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);

                    // Store the twist after the move
                    u_edges_move[N_MOVE * i + 3 * c1 as usize + k1] = a.get_u_edges() as u16;
                }
                // 4th move restores the face
                a.edge_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);
            }
        }
        // Write to file
        let mut file = File::create(path).expect("Unable to create file");
        for value in u_edges_move {
            file.write_u16::<LittleEndian>(value)
                .expect("Unable to write data");
        }
    } else {
        // If the file exists, load it
        println!("loading {} table...", fname);
        let mut file = File::open(path).expect("Unable to open file");
        let mut u_edges_move = vec![0u16; N_SLICE_SORTED * N_MOVE];

        // Read from the file
        for i in 0..(N_SLICE_SORTED * N_MOVE) {
            u_edges_move[i] = file
                .read_u16::<LittleEndian>()
                .expect("Unable to read data");
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Move table for the d_edges coordinate for transition phase 1 -> phase 2//
    ////////////////////////////////////////////////////////////////////////////
    // The d_edges coordinate describes the 12!/8! = 11880 possible positions of the DR, DF, DL and DB edges.
    // It is needed at the end of phase 1 to set up the coordinates of phase 2
    // 0 <= d_edges <= 11880 in phase 1, 0 <= d_edges <= 1680 in phase 2, d_edges = 0 for solved cube.
    let fname = "move_d_edges";
    let path = Path::new(FOLDER).join(fname);

    if !path.exists() {
        println!("creating {} table...", fname);
        let mut d_edges_move = vec![0u16; N_SLICE_SORTED * N_MOVE];
        let mut a = cb::CubieCube::new(None, None, None, None);
        for i in 0..N_SLICE_SORTED {
            if i % 200 == 0 {
                print!(".");
            }
            a.set_d_edges(i);
            for &c1 in enums::Color::iter() {
                for k1 in 0..3 {
                    // three moves for each face
                    // Apply the basic move (corner_multiply)
                    a.edge_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);

                    // Store the twist after the move
                    d_edges_move[N_MOVE * i + 3 * c1 as usize + k1] = a.get_d_edges() as u16;
                }
                // 4th move restores the face
                a.edge_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);
            }
        }
        // Write to file
        let mut file = File::create(path).expect("Unable to create file");
        for value in d_edges_move {
            file.write_u16::<LittleEndian>(value)
                .expect("Unable to write data");
        }
    } else {
        // If the file exists, load it
        println!("loading {} table...", fname);
        let mut file = File::open(path).expect("Unable to open file");
        let mut d_edges_move = vec![0u16; N_SLICE_SORTED * N_MOVE];

        // Read from the file
        for i in 0..(N_SLICE_SORTED * N_MOVE) {
            d_edges_move[i] = file
                .read_u16::<LittleEndian>()
                .expect("Unable to read data");
        }
    }

    ////////////////////////////////////////////////////////
    // Move table for the edges in the U-face and D-face. //
    ////////////////////////////////////////////////////////
    // The ud_edges coordinate describes the 40320 permutations of the edges UR, UF, UL, UB, DR, DF, DL and DB.
    // ud_edges undefined in phase 1, 0 <= ud_edges < 40320 in phase 2, ud_edges = 0 for solved cube.
    let fname = "move_ud_edges";
    let path = Path::new(FOLDER).join(fname);

    if !path.exists() {
        println!("creating {} table...", fname);
        let mut ud_edges_move = vec![0u16; N_UD_EDGES * N_MOVE];
        let mut a = cb::CubieCube::new(None, None, None, None);
        for i in 0..N_UD_EDGES {
            if (i + 1) % 600 == 0 {
                print!(".");
            }
            if (i + 1) % 48000 == 0 {
                print!("");
            }
            a.set_ud_edges(i);
            for &c1 in enums::Color::iter() {
                for k1 in 0..3 {
                    // three moves for each face
                    // Apply the basic move (corner_multiply)
                    a.edge_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);
                    if matches!(
                        c1,
                        enums::Color::R | enums::Color::F | enums::Color::L | enums::Color::B
                    ) && k1 != 1
                    {
                        continue;
                    }
                    // Store the twist after the move
                    ud_edges_move[N_MOVE * i + 3 * c1 as usize + k1] = a.get_ud_edges() as u16;
                }
                // 4th move restores the face
                a.edge_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);
            }
        }
        // Write to file
        let mut file = File::create(path).expect("Unable to create file");
        for value in ud_edges_move {
            file.write_u16::<LittleEndian>(value)
                .expect("Unable to write data");
        }
    } else {
        // If the file exists, load it
        println!("loading {} table...", fname);
        let mut file = File::open(path).expect("Unable to open file");
        let mut ud_edges_move = vec![0u16; N_UD_EDGES * N_MOVE];

        // Read from the file
        for i in 0..(N_UD_EDGES * N_MOVE) {
            ud_edges_move[i] = file
                .read_u16::<LittleEndian>()
                .expect("Unable to read data");
        }
    }

    //////////////////////////////////////////////////////
    // Move table for the corners coordinate in phase 2 //
    //////////////////////////////////////////////////////
    // The corners coordinate descirbes the 8! = 40320 permutations of the corners.
    // 0 <= corners < 40320 defined but unused in phase 1, 0 <= corners < 40320 in phase 2, corners = 0 for solved cube

    let fname = "move_corners";
    let path = Path::new(FOLDER).join(fname);

    if !path.exists() {
        println!("creating {} table...", fname);
        let mut corners_move = vec![0u16; N_CORNERS * N_MOVE];
        let mut a = cb::CubieCube::new(None, None, None, None);
        for i in 0..N_CORNERS {
            if (i + 1) % 200 == 0 {
                print!(".");
            }
            if (i + 1) % 16000 == 0 {
                print!("");
            }
            a.set_corners(i);
            for &c1 in enums::Color::iter() {
                for k1 in 0..3 {
                    // three moves for each face
                    // Apply the basic move (corner_multiply)
                    a.corner_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);
                    // Store the twist after the move
                    corners_move[N_MOVE * i + 3 * c1 as usize + k1] = a.get_corners() as u16;
                }
                // 4th move restores the face
                a.corner_multiply(&cb::BASIC_MOVE_CUBE[c1 as usize]);
            }
        }
        // Write to file
        let mut file = File::create(path).expect("Unable to create file");
        for value in corners_move {
            file.write_u16::<LittleEndian>(value)
                .expect("Unable to write data");
        }
    } else {
        // If the file exists, load it
        println!("loading {} table...", fname);
        let mut file = File::open(path).expect("Unable to open file");
        let mut corners_move = vec![0u16; N_CORNERS * N_MOVE];

        // Read from the file
        for i in 0..(N_CORNERS * N_MOVE) {
            corners_move[i] = file
                .read_u16::<LittleEndian>()
                .expect("Unable to read data");
        }
    }
}
