/// The cube on the cubie level is described by the permutation and orientations of corners and edges
use super::defs::{CORNER_COLOR, CORNER_FACELET, EDGE_COLOR, EDGE_FACELET};
use super::defs::{CO_B, CO_D, CO_F, CO_L, CO_R, CO_U, CP_B, CP_D, CP_F, CP_L, CP_R, CP_U};
use super::defs::{EO_B, EO_D, EO_F, EO_L, EO_R, EO_U, EP_B, EP_D, EP_F, EP_L, EP_R, EP_U};
use super::defs::{TURN_B, TURN_D, TURN_F, TURN_L, TURN_R, TURN_U};
use super::enums::{Color, Corner as Co, Edge as Ed};
use super::face::FaceCube;
use super::misc::{c_nk, rotate_left, rotate_right};

// Struct defining the Cube on the Cubie Level
#[derive(Debug, Clone, Copy)]
pub struct CubieCube {
    pub cp: [Co; 8],  // Corner permutation
    pub co: [u8; 8],  // Corner orientation
    pub ep: [Ed; 12], // Edge permutation
    pub eo: [u8; 12], // Edge orientation
}

impl CubieCube {
    pub fn new(
        cp: Option<[Co; 8]>,
        co: Option<[u8; 8]>,
        ep: Option<[Ed; 12]>,
        eo: Option<[u8; 12]>,
    ) -> Self {
        CubieCube {
            cp: cp.unwrap_or_else(|| {
                let mut default_cp = [Co::URF; 8];
                for (i, corner) in default_cp.iter_mut().enumerate() {
                    *corner = Co::from_usize(i).unwrap();
                }
                default_cp
            }),
            co: co.unwrap_or([0; 8]),
            ep: ep.unwrap_or_else(|| {
                let mut default_ep = [Ed::UR; 12];
                for (i, edge) in default_ep.iter_mut().enumerate() {
                    *edge = Ed::from_usize(i).unwrap();
                }
                default_ep
            }),
            eo: eo.unwrap_or([0; 12]),
        }
    }

    // Create an instance of the CubieCube from the standard Scramble-notation, e.g
    // "R F2 D' U2 B2 L D'..."
    pub fn from_scramble(scramble: &str) -> Self {
        let mut cube = CubieCube::new(None, None, None, None);

        let moves = scramble.split_whitespace();
        for mv in moves {
            match mv {
                "U" => cube.multiply(&TURN_U),
                "U2" => {
                    cube.multiply(&TURN_U);
                    cube.multiply(&TURN_U);
                }
                "U'" => {
                    cube.multiply(&TURN_U);
                    cube.multiply(&TURN_U);
                    cube.multiply(&TURN_U);
                }
                "R" => cube.multiply(&TURN_R),
                "R2" => {
                    cube.multiply(&TURN_R);
                    cube.multiply(&TURN_R);
                }
                "R'" => {
                    cube.multiply(&TURN_R);
                    cube.multiply(&TURN_R);
                    cube.multiply(&TURN_R);
                }
                "F" => cube.multiply(&TURN_F),
                "F2" => {
                    cube.multiply(&TURN_F);
                    cube.multiply(&TURN_F);
                }
                "F'" => {
                    cube.multiply(&TURN_F);
                    cube.multiply(&TURN_F);
                    cube.multiply(&TURN_F);
                }
                "D" => cube.multiply(&TURN_D),
                "D2" => {
                    cube.multiply(&TURN_D);
                    cube.multiply(&TURN_D);
                }
                "D'" => {
                    cube.multiply(&TURN_D);
                    cube.multiply(&TURN_D);
                    cube.multiply(&TURN_D);
                }
                "L" => cube.multiply(&TURN_L),
                "L2" => {
                    cube.multiply(&TURN_L);
                    cube.multiply(&TURN_L);
                }
                "L'" => {
                    cube.multiply(&TURN_L);
                    cube.multiply(&TURN_L);
                    cube.multiply(&TURN_L);
                }
                "B" => cube.multiply(&TURN_B),
                "B2" => {
                    cube.multiply(&TURN_B);
                    cube.multiply(&TURN_B);
                }
                "B'" => {
                    cube.multiply(&TURN_B);
                    cube.multiply(&TURN_B);
                    cube.multiply(&TURN_B);
                }
                _ => panic!("Invalid move: {}", mv),
            }
        }

        cube
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..8 {
            s.push_str(&format!("({:?},{})", self.cp[i], self.co[i]));
        }
        s.push('\n');
        for i in 0..12 {
            s.push_str(&format!("({:?},{})", self.ep[i], self.eo[i]));
        }
        s
    }

    // Applying a permutation to the Cube can be seen as multiplication with a different permutation
    pub fn multiply(&mut self, b: &CubieCube) {
        self.corner_multiply(b);
        self.edge_multiply(b);
    }

    pub fn corner_multiply(&mut self, b: &CubieCube) {
        let mut c_perm = [Co::URF; 8];
        let mut c_ori = [0; 8];

        for c in 0..8 {
            let j = self.cp[b.cp[c] as usize];
            let ori_a = self.co[b.cp[c] as usize];
            let ori_b = b.co[c];

            let ori = if ori_a < 3 && ori_b < 3 {
                (ori_a + ori_b) % 3
            } else if ori_a < 3 && ori_b >= 3 {
                (ori_a + ori_b) % 3 + 3
            } else if ori_a >= 3 && ori_b < 3 {
                (ori_a - ori_b + 3) % 3 + 3
            } else {
                (ori_a - ori_b + 3) % 3
            };

            c_perm[c] = j;
            c_ori[c] = ori;
        }

        self.cp.copy_from_slice(&c_perm);
        self.co.copy_from_slice(&c_ori);
    }

    pub fn edge_multiply(&mut self, b: &CubieCube) {
        let mut e_perm = [Ed::UR; 12];
        let mut e_ori = [0; 12];

        for e in 0..12 {
            let j = self.ep[b.ep[e] as usize];
            e_perm[e] = j;
            e_ori[e] = (self.eo[b.ep[e] as usize] + b.eo[e]) % 2;
        }

        self.ep.copy_from_slice(&e_perm);
        self.eo.copy_from_slice(&e_ori);
    }

    // Transform to the Facelet level
    pub fn to_facelet_cube(&self) -> FaceCube {
        let mut fc = FaceCube::new();

        for (i, &c) in CORNER_FACELET.iter().enumerate() {
            let corner = self.cp[i] as usize;
            let ori = self.co[i] as usize;

            for k in 0..3 {
                fc.f[c[(k + ori) % 3] as usize] = CORNER_COLOR[corner][k];
            }
        }

        for (i, &e) in EDGE_FACELET.iter().enumerate() {
            let edge = self.ep[i] as usize;
            let ori = self.eo[i] as usize;

            for k in 0..2 {
                fc.f[e[(k + ori) % 2] as usize] = EDGE_COLOR[edge][k];
            }
        }

        fc
    }

    // Let A be some scrambled Cube and B its inverse, then A * B = I (solved cube)
    // If we are looking for a solution of some scrambled cube, we are in fact looking
    // for its inverse permutation composed as a product of the permutations
    // corresponding to the elementary moves.
    pub fn inv_cubie_cube(&self, d: &mut CubieCube) {
        // Invert edge permutation and orientation
        for e in 0..12 {
            d.ep[self.ep[e] as usize] = Ed::from_usize(e).unwrap();
        }
        for e in 0..12 {
            d.eo[e] = self.eo[d.ep[e] as usize];
        }

        // Invert corner permutation and orientation
        for c in 0..8 {
            d.cp[self.cp[c] as usize] = Co::from_usize(c).unwrap();
        }
        for c in 0..8 {
            let ori = self.co[d.cp[c] as usize];
            if ori >= 3 {
                d.co[c] = ori;
            } else {
                d.co[c] = (3 - ori) % 3;
            }
        }
    }

    pub fn corner_parity(&self) -> u8 {
        let mut s = 0;
        for i in (1..8).rev() {
            for j in 0..i {
                if self.cp[j] > self.cp[i] {
                    s += 1;
                }
            }
        }
        s % 2
    }

    pub fn edge_parity(&self) -> u8 {
        let mut s = 0;
        for i in (1..12).rev() {
            for j in 0..i {
                if self.ep[j] > self.ep[i] {
                    s += 1;
                }
            }
        }
        s % 2
    }

    // Methods for getting the coordinates from the Cube to represent it on a coordinate level

    // Get the twist of the 8 corners. 0 <= twist <= 2187 in phase 1, twist = 0 in phase 2
    pub fn get_twist(&self) -> u16 {
        let mut twist: u16 = 0;
        for &co in self.co.iter().take(7) {
            twist = 3 * twist + co as u16;
        }
        twist
    }

    pub fn set_twist(&mut self, mut twist: u16) {
        let mut twist_parity: u16 = 0;

        for i in (0..7).rev() {
            self.co[i] = (twist % 3) as u8;
            twist_parity += self.co[i] as u16;
            twist /= 3;
        }
        self.co[7] = ((3 - (twist_parity % 3)) % 3) as u8;
    }

    // Get the flip of the 12 edges. 0 <= flip < 2048 in phase 1, flip = 0 in phase 2
    pub fn get_flip(&self) -> u16 {
        let mut flip: u16 = 0;
        for &eo in self.eo.iter().take(11) {
            flip = 2 * flip + eo as u16;
        }
        flip
    }

    pub fn set_flip(&mut self, mut flip: u16) {
        let mut flip_parity: u16 = 0;

        for i in (0..11).rev() {
            self.eo[i] = (flip % 2) as u8;
            flip_parity += self.eo[i] as u16;
            flip /= 2;
        }
        self.eo[11] = ((2 - (flip_parity % 2)) % 2) as u8;
    }

    // Get the location of the UD-slice edges FR,FL,BL and BR ignoring their permutation.
    // 0<=slice<495 in phase 1, slice = 0 in phase 2
    pub fn get_slice(&self) -> u16 {
        let mut a = 0;
        let mut x = 0;
        // Compute the index a < (12 choose 4)
        for j in (Ed::UR as usize..=Ed::BR as usize).rev() {
            if (Ed::FR..=Ed::BR).contains(&(self.ep[j])) {
                a += c_nk(11 - j, x + 1);
                x += 1;
            }
        }
        a as u16
    }

    pub fn set_slice(&mut self, idx: usize) {
        let slice_edge = [Ed::FR, Ed::FL, Ed::BL, Ed::BR];
        let other_edge = [
            Ed::UR,
            Ed::UF,
            Ed::UL,
            Ed::UB,
            Ed::DR,
            Ed::DF,
            Ed::DL,
            Ed::DB,
        ];
        let mut a = idx; // Location

        // Invalidate all edge positions
        for e in 0..self.ep.len() {
            self.ep[e] = Ed::Invalid; // Use a specific invalid edge representation
        }

        let mut x = 4; // Set slice edges
        for j in 0..12 {
            if a >= c_nk(11 - j, x) {
                self.ep[j] = slice_edge[4 - x];
                a -= c_nk(11 - j, x);
                x -= 1;
            }
        }

        let mut x = 0; // Set the remaining edges UR..DB
        for j in 0..12 {
            if self.ep[j] == Ed::Invalid {
                self.ep[j] = other_edge[x];
                x += 1;
            }
        }
    }

    pub fn get_slice_sorted(&self) -> usize {
        let mut a = 0;
        let mut x = 0;
        let mut edge4 = [0; 4];

        // Compute the index a < (12 choose 4) and the permutation array
        for j in (Ed::UR as usize..=Ed::BR as usize).rev() {
            if (Ed::FR..=Ed::BR).contains(&self.ep[j]) {
                a += c_nk(11 - j, x + 1);
                edge4[3 - x] = self.ep[j] as usize;
                x += 1;
            }
        }

        // Compute the index b < 4! for the permutation in edge4
        let mut b = 0;
        for j in (1..=3).rev() {
            let mut k = 0;
            while edge4[j] != (j + 8) {
                rotate_left(&mut edge4, 0, j); // Function to rotate left
                k += 1;
            }
            b = (j + 1) * b + k;
        }

        24 * a + b
    }

    pub fn set_slice_sorted(&mut self, idx: usize) {
        let mut slice_edge = vec![Ed::FR, Ed::FL, Ed::BL, Ed::BR];
        let other_edge = vec![
            Ed::UR,
            Ed::UF,
            Ed::UL,
            Ed::UB,
            Ed::DR,
            Ed::DF,
            Ed::DL,
            Ed::DB,
        ];

        let mut b = idx % 24; // Permutation
        let mut a = idx / 24; // Location

        for e in Ed::iter() {
            self.ep[e as usize] = Ed::Invalid; // Invalidate all edge positions
        }

        // Generate permutation from index b
        let mut j = 1;
        while j < 4 {
            let mut k = b % (j + 1);
            b /= j + 1;
            while k > 0 {
                rotate_right(&mut slice_edge, 0, j); // Function to rotate right
                k -= 1;
            }
            j += 1;
        }

        // Set slice edges
        let mut x = 4;
        for j in 0..12 {
            if a >= c_nk(11 - j, x) {
                self.ep[j] = slice_edge[4 - x];
                a -= c_nk(11 - j, x);
                x -= 1;
            }
        }

        // Set the remaining edges
        let mut x = 0;
        for j in 0..12 {
            if self.ep[j] == Ed::Invalid {
                self.ep[j] = other_edge[x];
                x += 1;
            }
        }
    }

    pub fn get_u_edges(&self) -> usize {
        let mut a = 0;
        let mut x = 0;
        let mut edge4 = [0; 4];
        let mut ep_mod = self.ep.clone();

        // Rotate ep_mod right 4 times
        for _ in 0..4 {
            rotate_right(&mut ep_mod, 0, 11); // Implement rotate_right as a helper function
        }

        // Compute the index a < (12 choose 4) and the permutation array
        for j in (Ed::UR as usize..=Ed::BR as usize).rev() {
            if (Ed::UR..=Ed::UB).contains(&ep_mod[j]) {
                a += c_nk(11 - j, x + 1);
                edge4[3 - x] = ep_mod[j] as usize;
                x += 1;
            }
        }

        // Compute the index b < 4! for the permutation in edge4
        let mut b = 0;
        for j in (1..=3).rev() {
            let mut k = 0;
            while edge4[j] != j {
                rotate_left(&mut edge4, 0, j); // Implement rotate_left as a helper function
                k += 1;
            }
            b = (j + 1) * b + k;
        }

        24 * a + b
    }

    pub fn set_u_edges(&mut self, idx: usize) {
        let mut slice_edge = vec![Ed::UR, Ed::UF, Ed::UL, Ed::UB];
        let other_edge = vec![
            Ed::DR,
            Ed::DF,
            Ed::DL,
            Ed::DB,
            Ed::FR,
            Ed::FL,
            Ed::BL,
            Ed::BR,
        ];

        let mut b = idx % 24; // Permutation
        let mut a = idx / 24; // Location

        for e in Ed::iter() {
            self.ep[e as usize] = Ed::Invalid; // Assuming Edge::INVALID exists
        }

        // Generate permutation from index b
        let mut j = 1;
        while j < 4 {
            let mut k = b % (j + 1);
            b /= j + 1;
            while k > 0 {
                rotate_right(&mut slice_edge, 0, j); // Rotate slice_edge to the right
                k -= 1;
            }
            j += 1;
        }

        // Set slice edges
        let mut x = 4;
        for j in 0..12 {
            if a >= c_nk(11 - j, x) {
                self.ep[j] = slice_edge[4 - x];
                a -= c_nk(11 - j, x);
                x -= 1;
            }
        }

        // Set the remaining edges
        let mut x = 0;
        for j in 0..12 {
            if self.ep[j] == Ed::Invalid {
                // Assuming Edge::INVALID exists
                self.ep[j] = other_edge[x];
                x += 1;
            }
        }

        // Rotate ep back to original order
        for _ in 0..4 {
            rotate_left(&mut self.ep, 0, 11); // Rotate the edges back to the original position
        }
    }

    pub fn get_d_edges(&self) -> usize {
        let mut a = 0;
        let mut x = 0;
        let mut edge4 = [0; 4];
        let mut ep_mod = self.ep.clone();

        // Rotate ep_mod right 4 times
        for _ in 0..4 {
            rotate_right(&mut ep_mod, 0, 11); // Implement rotate_right as a helper function
        }

        // Compute the index a < (12 choose 4) and the permutation array
        for j in (Ed::UR as usize..=Ed::BR as usize).rev() {
            if (Ed::DR..=Ed::DB).contains(&ep_mod[j]) {
                a += c_nk(11 - j, x + 1);
                edge4[3 - x] = ep_mod[j] as usize;
                x += 1;
            }
        }

        // Compute the index b < 4! for the permutation in edge4
        let mut b = 0;
        for j in (1..=3).rev() {
            let mut k = 0;
            while edge4[j] != (j + 4) {
                rotate_left(&mut edge4, 0, j); // Implement rotate_left as a helper function
                k += 1;
            }
            b = (j + 1) * b + k;
        }

        24 * a + b
    }

    pub fn set_d_edges(&mut self, idx: usize) {
        let mut slice_edge = vec![Ed::DR, Ed::DF, Ed::DL, Ed::DB];
        let other_edge = vec![
            Ed::FR,
            Ed::FL,
            Ed::BL,
            Ed::BR,
            Ed::UR,
            Ed::UF,
            Ed::UL,
            Ed::UB,
        ];

        let mut b = idx % 24; // Permutation
        let mut a = idx / 24; // Location

        for e in Ed::iter() {
            self.ep[e as usize] = Ed::Invalid; // Assuming Edge::INVALID exists
        }

        // Generate permutation from index b
        let mut j = 1;
        while j < 4 {
            let mut k = b % (j + 1);
            b /= j + 1;
            while k > 0 {
                rotate_right(&mut slice_edge, 0, j); // Rotate slice_edge to the right
                k -= 1;
            }
            j += 1;
        }

        // Set slice edges
        let mut x = 4;
        for j in 0..12 {
            if a >= c_nk(11 - j, x) {
                self.ep[j] = slice_edge[4 - x];
                a -= c_nk(11 - j, x);
                x -= 1;
            }
        }

        // Set the remaining edges
        let mut x = 0;
        for j in 0..12 {
            if self.ep[j] == Ed::Invalid {
                // Assuming Edge::INVALID exists
                self.ep[j] = other_edge[x];
                x += 1;
            }
        }

        // Rotate ep back to original order
        for _ in 0..4 {
            rotate_left(&mut self.ep, 0, 11); // Rotate the edges back to the original position
        }
    }

    /// Get the permutation of the 8 corners.
    /// 0 <= corners < 40320 defined but unused in phase 1, 0 <= corners < 40320 in phase 2,
    /// corners = 0 for solved cube
    pub fn get_corners(&self) -> usize {
        let mut perm = self.cp.clone();
        let mut b = 0;
        for j in (Co::UFL as usize..=Co::DRB as usize).rev() {
            let mut k = 0;

            for i in (Co::URF as usize..j).rev() {
                if self.cp[i] > self.cp[j] {
                    k += 1;
                }
            }
            b = (b + k) * j as usize;
        }
        b
    }

    pub fn set_corners(&mut self, mut idx: usize) {
        for i in Co::URF as usize..=Co::DRB as usize {
            self.cp[i] = Co::from_usize(i).expect("Invalid edge index");
        }
        for j in 0..=7 {
            let mut k = idx % (j + 1);
            idx /= j + 1;
            while k > 0 {
                rotate_right(&mut self.cp, 0, j);
                k -= 1;
            }
        }
    }

    /// Get the permutation of the 8 U and D edges.
    pub fn get_ud_edges(&self) -> usize {
        let mut b = 0;
        for j in (Ed::UF as usize..=Ed::DB as usize).rev() {
            let mut k = 0;
            for i in (Ed::UR as usize..j).rev() {
                if self.ep[i] > self.ep[j] {
                    k += 1;
                }
            }
            b = (b + k) * j as usize;
        }
        b
    }

    // This doesn't work properly yet!
    pub fn set_ud_edges(&mut self, mut idx: usize) {
        let mut idx = idx;
        for j in 0..8 {
            let mut k = idx % (j + 1);
            idx /= j + 1;
            while k > 0 {
                rotate_right(&mut self.ep, 0, j);
                k -= 1;
            }
        }
    }
}

impl std::fmt::Display for CubieCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for CubieCube {
    fn eq(&self, other: &Self) -> bool {
        self.cp == other.cp && self.co == other.co && self.ep == other.ep && self.eo == other.eo
    }
}

// Utility trait to convert usize to enums for initialization
pub trait FromUsize {
    fn from_usize(value: usize) -> Option<Self>
    where
        Self: Sized;
}

impl FromUsize for Co {
    fn from_usize(value: usize) -> Option<Self> {
        match value {
            0 => Some(Co::URF),
            1 => Some(Co::UFL),
            2 => Some(Co::ULB),
            3 => Some(Co::UBR),
            4 => Some(Co::DFR),
            5 => Some(Co::DLF),
            6 => Some(Co::DBL),
            7 => Some(Co::DRB),
            _ => None,
        }
    }
}

impl FromUsize for Ed {
    fn from_usize(value: usize) -> Option<Self> {
        match value {
            0 => Some(Ed::UR),
            1 => Some(Ed::UF),
            2 => Some(Ed::UL),
            3 => Some(Ed::UB),
            4 => Some(Ed::DR),
            5 => Some(Ed::DF),
            6 => Some(Ed::DL),
            7 => Some(Ed::DB),
            8 => Some(Ed::FR),
            9 => Some(Ed::FL),
            10 => Some(Ed::BL),
            11 => Some(Ed::BR),
            _ => None,
        }
    }
}
const NUM_MOVES: usize = 18;

lazy_static::lazy_static! {
    pub static ref BASIC_MOVE_CUBE: [CubieCube; 6] = {
        let mut cubes = [CubieCube::new(None, None, None, None); 6];
        cubes[Color::U as usize] = CubieCube::new(Some(CP_U), Some(CO_U), Some(EP_U), Some(EO_U));
        cubes[Color::R as usize] = CubieCube::new(Some(CP_R), Some(CO_R), Some(EP_R), Some(EO_R));
        cubes[Color::F as usize] = CubieCube::new(Some(CP_F), Some(CO_F), Some(EP_F), Some(EO_F));
        cubes[Color::D as usize] = CubieCube::new(Some(CP_D), Some(CO_D), Some(EP_D), Some(EO_D));
        cubes[Color::L as usize] = CubieCube::new(Some(CP_L), Some(CO_L), Some(EP_L), Some(EO_L));
        cubes[Color::B as usize] = CubieCube::new(Some(CP_B), Some(CO_B), Some(EP_B), Some(EO_B));
        cubes
    };
}

lazy_static::lazy_static! {
    pub static ref MOVE_CUBE: [CubieCube; NUM_MOVES] = {
        let mut move_cube = [CubieCube::new(None, None, None, None); NUM_MOVES];

        for &c1 in Color::iter() {
            let mut cc = CubieCube::new(None, None, None, None);
            for k1 in 0..3 {
                cc.multiply(&BASIC_MOVE_CUBE[c1 as usize]); // Assuming `multiply` mutates the object
                move_cube[3 * c1 as usize + k1] = CubieCube::new(
                    Some(cc.cp),
                    Some(cc.co),
                    Some(cc.ep),
                    Some(cc.eo),
                );
            }
        }

        move_cube
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_twist() {
        // test get_twist
        let cube = CubieCube::new(None, None, None, None);
        assert_eq!(cube.get_twist(), 0);
        let cube = CubieCube::from_scramble("R");
        assert_eq!(cube.get_twist(), 1494);
        let cube = CubieCube::from_scramble("R F2 U B L");
        assert_eq!(cube.get_twist(), 1410);
        let cube =
            CubieCube::from_scramble("R U R' U' R U R' U' R U R' U' R U R' U' R U R' U' R U R' U'");
        assert_eq!(cube.get_twist(), 0);
        // test set twist
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_twist(324);
        assert_eq!(cube.get_twist(), 324);
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_twist(2180);
        assert_eq!(cube.get_twist(), 2180);
    }

    #[test]
    fn test_flip() {
        // test get_flip
        let cube = CubieCube::from_scramble("R");
        assert_eq!(cube.get_flip(), 0);
        let cube = CubieCube::from_scramble("F");
        assert_eq!(cube.get_flip(), 550);
        let cube = CubieCube::from_scramble("R F");
        assert_eq!(cube.get_flip(), 550);
        let cube = CubieCube::from_scramble("B' U F D L2 F'");
        assert_eq!(cube.get_flip(), 1349);
        // test set_flip
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_flip(1000);
        assert_eq!(cube.get_flip(), 1000);
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_flip(691);
        assert_eq!(cube.get_flip(), 691);
    }

    #[test]
    fn test_get_slice() {
        let cube = CubieCube::from_scramble("R U L' R2 U' L2");
        let ud_slice = cube.get_slice();
        assert_eq!(ud_slice, 494);
        let cube = CubieCube::from_scramble("R' D R");
        let ud_slice = cube.get_slice();
        assert_eq!(ud_slice, 1);
    }

    #[test]
    fn test_set_slice() {
        let mut cube = CubieCube::new(None, None, None, None); // Assuming default initializes a solved cube
        cube.set_slice(494);

        // Verify that `get_slice` returns the same value as was set
        let ud_slice = cube.get_slice();
        assert_eq!(ud_slice, 494);

        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_slice(1);

        // Verify that `get_slice` returns the same value as was set
        let ud_slice = cube.get_slice();
        assert_eq!(ud_slice, 1);
    }

    #[test]
    fn test_slice_sorted() {
        let mut cube = CubieCube::new(None, None, None, None); // Assuming default initializes a solved cube

        // Test set_slice_sorted and get_slice_sorted with various indices
        let test_cases = vec![
            (0, "Solved cube"),      // Solved cube should have index 0
            (11879, "Max index"),    // Maximum index for phase 1
            (24, "Phase 2 example"), // Example for phase 2
            (494, "Random index"),   // Random valid index
        ];

        for (idx, description) in test_cases {
            cube.set_slice_sorted(idx);

            // After setting, the slice index retrieved should match the one set
            let calculated_idx = cube.get_slice_sorted();
            assert_eq!(calculated_idx, idx, "Failed on test case: {}", description);
        }
    }

    #[test]
    fn test_u_edges() {
        let mut cube = CubieCube::new(None, None, None, None); // Assuming default initializes a solved cube

        // Test cases: (index, description)
        let test_cases = vec![
            (1656, "Solved cube"),       // Solved cube should return 1656
            (0, "Minimum index"),        // Minimum valid index
            (1679, "Maximum index"),     // Maximum index for phase 2
            (850, "Random valid index"), // Random valid index
        ];

        for (idx, description) in test_cases {
            // Set the U edges using the index
            cube.set_u_edges(idx);

            // Retrieve the index using get_u_edges
            let calculated_idx = cube.get_u_edges();

            // Assert that the calculated index matches the original
            assert_eq!(calculated_idx, idx, "Failed on test case: {}", description);
        }
    }

    #[test]
    fn test_d_edges() {
        let mut cube = CubieCube::new(None, None, None, None); // Assuming default initializes a solved cube

        // Test cases: (index, description)
        let test_cases = vec![
            (0, "Solved cube"),          // Solved cube should return 0
            (1656, "Phase 2 example"),   // Example for phase 2
            (1679, "Maximum index"),     // Maximum index for phase 2
            (850, "Random valid index"), // Random valid index
        ];

        for (idx, description) in test_cases {
            // Set the D edges using the index
            cube.set_d_edges(idx);

            // Retrieve the index using get_d_edges
            let calculated_idx = cube.get_d_edges();

            // Assert that the calculated index matches the original
            assert_eq!(calculated_idx, idx, "Failed on test case: {}", description);
        }
    }

    #[test]
    fn test_corners() {
        let mut cube = CubieCube::from_scramble("R");
        assert_eq!(cube.get_corners(), 21021);
        let mut cube = CubieCube::from_scramble("R F");
        assert_eq!(cube.get_corners(), 20924);
        let mut cube = CubieCube::from_scramble("R R'");
        assert_eq!(cube.get_corners(), 0);

        let mut cube2 = CubieCube::new(None, None, None, None);
        cube2.set_corners(21021);
        assert_eq!(cube2.get_corners(), 21021);
    }

    #[test]
    fn ud_edges() {
        let cube = CubieCube::from_scramble("R2 U L2");
        assert_eq!(cube.get_ud_edges(), 3834);
        let mut cube2 = CubieCube::from_scramble("");
        cube2.set_ud_edges(105);
        assert_eq!(cube2.get_ud_edges(), 3834);
    }
}
