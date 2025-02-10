use super::bfs::{CORNER_DB, EDGE_DB};
use super::defs::N_SYM;
use super::defs::{CORNER_COLOR, CORNER_FACELET, EDGE_COLOR, EDGE_FACELET};
use super::defs::{CO_B, CO_D, CO_F, CO_L, CO_R, CO_U, CP_B, CP_D, CP_F, CP_L, CP_R, CP_U};
use super::defs::{EO_B, EO_D, EO_F, EO_L, EO_R, EO_U, EP_B, EP_D, EP_F, EP_L, EP_R, EP_U};
use super::defs::{TURN_B, TURN_D, TURN_F, TURN_L, TURN_R, TURN_U};
use super::enums::{Color, Corner as Co, Edge as Ed};
use super::face::FaceCube;
use super::misc::{c_nk, factorial, rotate_left, rotate_right};
use super::symmetries::{INV_IDX, SYM_CUBE};
use num::integer::binomial;
use rand::seq::SliceRandom;
use std::hash::{Hash, Hasher};

/// The cube on the cubie level is described by the permutation and orientations of corners and edges
#[derive(Debug, Clone, Copy, Eq)]
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

    pub fn is_solved(&self) -> bool {
        let solved_state = Self::new(None, None, None, None);
        self.cp == solved_state.cp
            && self.co == solved_state.co
            && self.ep == solved_state.ep
            && self.eo == solved_state.eo
    }

    /// Create an instance of the CubieCube from the standard Scramble-notation, e.g
    /// "R F2 D' U2 B2 L D'..."
    pub fn from_scramble(scramble: &str) -> Self {
        let mut cube = CubieCube::new(None, None, None, None);

        let moves = scramble.split_whitespace();
        for mv in moves {
            match mv {
                "U" | "U1" => cube.multiply(&TURN_U),
                "U2" => {
                    cube.multiply(&TURN_U);
                    cube.multiply(&TURN_U);
                }
                "U'" | "U3" => {
                    cube.multiply(&TURN_U);
                    cube.multiply(&TURN_U);
                    cube.multiply(&TURN_U);
                }
                "R" | "R1" => cube.multiply(&TURN_R),
                "R2" => {
                    cube.multiply(&TURN_R);
                    cube.multiply(&TURN_R);
                }
                "R'" | "R3" => {
                    cube.multiply(&TURN_R);
                    cube.multiply(&TURN_R);
                    cube.multiply(&TURN_R);
                }
                "F" | "F1" => cube.multiply(&TURN_F),
                "F2" => {
                    cube.multiply(&TURN_F);
                    cube.multiply(&TURN_F);
                }
                "F'" | "F3" => {
                    cube.multiply(&TURN_F);
                    cube.multiply(&TURN_F);
                    cube.multiply(&TURN_F);
                }
                "D" | "D1" => cube.multiply(&TURN_D),
                "D2" => {
                    cube.multiply(&TURN_D);
                    cube.multiply(&TURN_D);
                }
                "D'" | "D3" => {
                    cube.multiply(&TURN_D);
                    cube.multiply(&TURN_D);
                    cube.multiply(&TURN_D);
                }
                "L" | "L1" => cube.multiply(&TURN_L),
                "L2" => {
                    cube.multiply(&TURN_L);
                    cube.multiply(&TURN_L);
                }
                "L'" | "L3" => {
                    cube.multiply(&TURN_L);
                    cube.multiply(&TURN_L);
                    cube.multiply(&TURN_L);
                }
                "B" | "B1" => cube.multiply(&TURN_B),
                "B2" => {
                    cube.multiply(&TURN_B);
                    cube.multiply(&TURN_B);
                }
                "B'" | "B3" => {
                    cube.multiply(&TURN_B);
                    cube.multiply(&TURN_B);
                    cube.multiply(&TURN_B);
                }
                _ => panic!("Invalid move: {}", mv),
            }
        }

        cube
    }

    /// Applying a permutation to the Cube can be seen as multiplication with a different permutation
    pub fn multiply(&mut self, b: &CubieCube) {
        self.corner_multiply(b);
        self.edge_multiply(b);
    }

    pub fn corner_multiply(&mut self, b: &CubieCube) {
        // Temporary arrays to hold the new permutation/orientation
        let mut c_perm: [Co; 8] = [Co::URF; 8];
        let mut c_ori = [0u8; 8];

        // For each corner index c in 0..8
        for c in 0..8 {
            // b.cp[c] = which corner is in slot c of b
            // So the corner in slot c of b is at index b.cp[c] in self.
            let corner_b = b.cp[c] as usize;

            // Build new corner permutation
            c_perm[c] = self.cp[corner_b];

            // Gather the orientation pieces
            let ori_a = self.co[corner_b]; // orientation from 'self'
            let ori_b = b.co[c]; // orientation from 'b'

            // Compute the new orientation
            let ori: i32 = if ori_a < 3 && ori_b < 3 {
                // Both corners are "regular"
                let mut result = ori_a as i32 + ori_b as i32;
                if result >= 3 {
                    result -= 3;
                }
                result
            } else if ori_a < 3 && ori_b >= 3 {
                // 'b' is in a mirrored state
                let mut result = ori_a as i32 + ori_b as i32;
                if result >= 6 {
                    result -= 3;
                }
                result
            } else if ori_a >= 3 && ori_b < 3 {
                // 'self' is in a mirrored state
                let mut result = ori_a as i32 - ori_b as i32;
                if result < 3 {
                    result += 3;
                }
                result
            } else {
                // Both are mirrored
                // (ori_a >= 3 && ori_b >= 3)
                let mut result = ori_a as i32 - ori_b as i32;
                if result < 0 {
                    result += 3;
                }
                result
            };

            // Store the result (cast back to u8)
            c_ori[c] = ori as u8;
        }

        // Now copy the temp results back into self
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

    /// Transform to the Facelet level
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

    /// Let A be some scrambled Cube and B its inverse, then A * B = I (solved cube)
    /// If we are looking for a solution of some scrambled cube, we are in fact looking
    /// for its inverse permutation composed as a product of the permutations
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

    /// Generate a list of the symmetries and antisymmetries of this cube.
    pub fn symmetries(&self) -> Vec<usize> {
        let mut s = Vec::new();
        let mut d = CubieCube::new(None, None, None, None);

        for j in 0..N_SYM {
            // Build the j-th symmetry cube
            let mut c = CubieCube::new(
                Some(SYM_CUBE[j].cp),
                Some(SYM_CUBE[j].co),
                Some(SYM_CUBE[j].ep),
                Some(SYM_CUBE[j].eo),
            );
            // c = symCube[j] * self * symCube[inv_idx[j]]
            c.multiply(self);
            c.multiply(&SYM_CUBE[INV_IDX[j] as usize]);

            // If self == c, we found a symmetry
            if *self == c {
                s.push(j);
            }

            // Now check for antisymmetry via c's inverse
            c.inv_cubie_cube(&mut d);
            if *self == d {
                // If the inverse of c equals self => we have antisymmetry
                s.push(j + N_SYM);
            }
        }

        s
    }

    // Methods for getting the coordinates from the Cube to represent it on a coordinate level

    /// Get the twist of the 8 corners. 0 <= twist <= 2187 in phase 1, twist = 0 in phase 2
    pub fn get_twist(&self) -> usize {
        let mut twist: u16 = 0;
        for &co in self.co.iter().take(7) {
            twist = 3 * twist + co as u16;
        }
        twist as usize
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

    /// Get the flip of the 12 edges. 0 <= flip < 2048 in phase 1, flip = 0 in phase 2
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

    /// Binomial coefficient function: C(n, k).
    /// For example, nCr. Returns 0 if k > n or k < 0.
    fn c(&self, n: i32, k: i32) -> u16 {
        if k < 0 || k > n {
            return 0;
        }

        binomial(n, k) as u16
    }

    pub fn get_slice(&self) -> usize {
        let mut a = 0usize; // Combination index
        let mut x = 0usize; // Count of UD-slice edges found so far

        // Check each edge in reverse (from BR=11 down to UR=0)
        for j in (Ed::UR as usize..=Ed::BR as usize).rev() {
            let e = self.ep[j];
            // If the edge is in the range [FR=8..BR=11], increment `a` and `x`
            if e >= Ed::FR && e <= Ed::BR {
                a += c_nk(11 - j, x + 1);
                x += 1;
            }
        }

        a
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

    pub fn get_slice_sorted(&self) -> u16 {
        let mut a = 0usize;
        let mut x = 0usize;
        let mut edge4 = [0u8; 4];

        // 1) Identify which positions in `ep` hold FR..BR (8..11).
        //    In descending order j = BR..UR => 11..=0
        //    If ep[j] is in [8..11], increment a by C(11-j, x+1) and store ep[j] in edge4.
        for j in (Ed::UR as usize..=Ed::BR as usize).rev() {
            let e = self.ep[j];
            if e >= Ed::FR && e <= Ed::BR {
                a += c_nk(11 - j, x + 1);
                edge4[3 - x] = e as u8; // fill from the back
                x += 1;
            }
        }

        let mut b = 0usize;
        for j in (1..=3).rev() {
            let mut k = 0usize;
            // The correct edge for index j is `j + 8`,
            // because FR=8, FL=9, BL=10, BR=11.
            while edge4[j] != (j as u8 + 8) {
                rotate_left(&mut edge4, 0, j);
                k += 1;
            }
            b = (j + 1) * b + k;
        }

        // 3) Return 24*a + b
        (24 * a + b) as u16
    }

    fn unrank_ud_permutation(&self, mut x: u16) -> [u8; 4] {
        // We'll store the four UD edges in a small "pool" vector,
        // then remove at index `digit` to build the permutation.
        let mut pool = vec![8u8, 9, 10, 11]; // [FR, FL, BL, BR]
        let mut perm = [0u8; 4];

        for i in (1..=4).rev() {
            let digit = (x % i) as usize;
            x /= i;
            perm[4 - i as usize] = pool.remove(digit);
        }
        perm
    }

    /// Sets `self`'s edges so that `ud_slice_sorted_coord()` will equal `coord`.
    /// 1) Decode `s = coord // 24` => which 4 of the 12 edges are UD-slice.
    /// 2) Decode `x = coord % 24` => the permutation of [FR,FL,BL,BR].
    /// 3) Write the resulting edges into `self.p_edge`.
    pub fn set_slice_sorted(&mut self, coord: u16) {
        // 1) Split `coord` into combination index `s` and permutation index `x`.
        let mut s = coord / 24; // this is the combination part
        let x = coord % 24; // this is the permutation part

        let mut occupied = [false; 12];
        let mut n: i32 = 11;
        let mut k: i32 = 3; // we want exactly 4 edges in the UD-slice

        while k >= 0 {
            let c_nk = self.c(n, k);
            if s >= c_nk {
                // means this position is NOT used
                s -= c_nk;
                occupied[n as usize] = false;
            } else {
                // means this position is used => UD-slice
                occupied[n as usize] = true;
                k -= 1;
            }
            n -= 1;
        }

        // 2) Decode the permutation of the 4 UD-slice edges using unranking.
        let ud_perm = self.unrank_ud_permutation(x);

        // 3) Now fill in `self.p_edge[i].e` for i in [0..11].
        //    - If occupied[i], we pick the next edge from `ud_perm`.
        //    - If not, we pick from the "non-UD" edges [0..7] in ascending order.
        //      That’s because in your code, edges < FR(=8) are UR,UF,UL,UB,DR,DF,DL,DB => 0..7.
        let mut ud_idx = 0; // index into ud_perm
        let mut non_ud_idx = 0;
        let non_ud_edges = [0u8, 1, 2, 3, 4, 5, 6, 7];

        for (i, item) in occupied.iter().enumerate() {
            if *item {
                // This position gets a UD-slice edge
                self.ep[i] = Ed::from_index(ud_perm[ud_idx] as usize);
                ud_idx += 1;
            } else {
                // This position gets a "non-UD" edge
                self.ep[i] = Ed::from_index(non_ud_edges[non_ud_idx] as usize);
                non_ud_idx += 1;
            }
        }
    }

    /// Returns a coordinate in [0..11880) for phase1, or [0..1680) in phase2, etc.
    pub fn get_u_edges(&self) -> u16 {
        // 1) Make a copy of ep and rotate it 4 times to the right on the slice [0..=11].
        let mut ep_mod = self.ep;
        for _ in 0..4 {
            rotate_right(&mut ep_mod, 0, 11);
        }

        // 2) Compute combination index `a` and gather the 4 U-edges into `edge4`.
        //    U-edges = UR, UF, UL, UB => Edge::UR=0..Edge::UB=3
        let mut a = 0usize;
        let mut x = 0usize;
        let mut edge4 = [0u8; 4];

        for j in (Ed::UR as usize..=Ed::BR as usize).rev() {
            let ed = ep_mod[j]; // e.g., might be 0..11
            if ed >= Ed::UR && ed <= Ed::UB {
                // edges UR..UB => 0..3
                a += c_nk(11 - j, x + 1);
                edge4[3 - x] = ed as u8;
                x += 1;
            }
        }

        let mut b = 0usize;
        for j in (1..=3).rev() {
            let mut k = 0usize;
            while edge4[j] != j as u8 {
                rotate_left(&mut edge4, 0, j);
                k += 1;
            }
            b = (j + 1) * b + k;
        }

        // 4) Return 24*a + b
        24 * a as u16 + b as u16
    }

    pub fn set_u_edges(&mut self, idx: usize) {
        let mut slice_edge = vec![Ed::UR, Ed::UF, Ed::UL, Ed::UB];
        let other_edge = [
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

    pub fn get_d_edges(&self) -> u16 {
        // Make a local copy, rotate it 4 times to the right on [0..=11].
        let mut ep_mod = self.ep;
        for _ in 0..4 {
            rotate_right(&mut ep_mod, 0, 11);
        }

        let mut a = 0usize;
        let mut x = 0usize;
        let mut edge4 = [0u8; 4];

        // 1) Compute combination index `a` and fill the 4 D-edges (DR..DB => 4..7) into edge4.

        for j in (Ed::UR as usize..=Ed::BR as usize).rev() {
            let ed = ep_mod[j];
            if ed >= Ed::DR && ed <= Ed::DB {
                a += c_nk(11 - j, x + 1);
                edge4[3 - x] = ed as u8;
                x += 1;
            }
        }

        // 2) Compute the permutation index `b` < 4!.

        let mut b = 0usize;
        for j in (1..=3).rev() {
            let mut k = 0usize;
            while edge4[j] != (j as u8 + 4) {
                rotate_left(&mut edge4, 0, j);
                k += 1;
            }
            b = (j + 1) * b + k;
        }

        // 3) Return 24*a + b
        24 * a as u16 + b as u16
    }

    pub fn set_d_edges(&mut self, idx: usize) {
        let mut slice_edge = vec![Ed::DR, Ed::DF, Ed::DL, Ed::DB];
        let other_edge = [
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
            self.ep[e as usize] = Ed::Invalid;
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
        // Duplicate the corner permutation array
        let mut perm = self.cp;
        let mut b = 0usize;

        // Iterate from DRB (last corner) to URF (first corner) in reverse order
        for j in (Co::URF as usize..=Co::DRB as usize).rev() {
            let mut k = 0;

            // Rotate left until perm[j] is in its correct position
            while perm[j] != Co::from_usize(j).unwrap() {
                rotate_left(&mut perm, 0, j);
                k += 1;
            }

            // Update the permutation index
            b = (j + 1) * b + k;
        }

        b
    }

    pub fn set_corners(&mut self, mut idx: usize) {
        // This method is divided in 2 steps. In step one we get the corresponding k
        // indices.
        // Set the initial values
        self.cp = Co::ALL;
        // Create a placeholder for values k
        let mut array_k = Vec::new();
        // Compute the values k
        for j in (1..=7).rev() {
            let k = idx / (factorial(j));
            array_k.push(k);
            idx %= factorial(j);
        }
        // Rotate with corresponding indices
        // create placeholder for the result
        let mut result = Vec::new();
        // Clone the original array of Corners to the Vector of Corners for
        // easier computation
        let mut array = Vec::new();
        for i in 0..self.cp.len() {
            array.push(self.cp[i]);
        }
        for j in array_k {
            let array_len = array.len();
            // Bring the right index to the end of the array
            for _ in 0..j {
                rotate_right(&mut array, 0, array_len - 1);
            }
            // Insert the last index to the first element of the result array
            result.insert(0, array[array.len() - 1]);
            // Remove the last index from the help-array and sort it
            array.pop();
            array.sort();
        }
        // Insert the last index to the first place
        result.insert(0, array.pop().unwrap());

        // Assign the result to the attribute
        self.cp[..(7 + 1)].copy_from_slice(&result[..(7 + 1)]);
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
            b = (b + k) * j;
        }
        b
    }

    pub fn set_ud_edges(&mut self, mut idx: usize) {
        // Create a placeholder for k values
        let mut array_k = Vec::new();
        // Compute k values
        for j in (1..=7).rev() {
            let k = idx / factorial(j);
            array_k.push(k);
            idx %= factorial(j);
        }

        // Rotate using the k indices
        let mut result = Vec::new();
        let mut array = Vec::new();
        // Clone the original ep array to a vector for easier manipulation
        for i in 0..self.ep.len() {
            array.push(self.ep[i]);
        }
        for j in array_k {
            let array_len = array.len();
            // Rotate to bring the correct index to the end of the array
            for _ in 0..j {
                rotate_right(&mut array, 0, array_len - 1);
            }
            // Insert the last element of the array into the result
            result.insert(0, array[array.len() - 1]);
            // Remove the last element and sort the remaining array
            array.pop();
            array.sort();
        }
        // Insert the final element into the result
        result.insert(0, array.pop().unwrap());

        // Assign the result to the ep attribute
        self.ep[..(7 + 1)].copy_from_slice(&result[..(7 + 1)]);
    }

    pub fn heuristic(&self) -> usize {
        let corner_h = (self.corner_manhattan_distance() + 3) / 4; // +3 for rounding up
        let edge_h = (self.edge_manhattan_distance() + 3) / 4; // +3 for rounding up

        corner_h.max(edge_h)
    }

    pub fn corner_manhattan_distance(&self) -> usize {
        let mut distance = 0;
        for (corner_index, (cp, co)) in self.cp.iter().zip(&self.co).enumerate() {
            let permutation = *cp as usize;
            let orientation = *co as usize;

            let index = permutation * 3 * 8 + corner_index * 3 + orientation;
            distance += CORNER_DB[index];
        }
        distance as usize
    }

    pub fn edge_manhattan_distance(&self) -> usize {
        let mut distance = 0;
        for (edge_index, (ep, eo)) in self.ep.iter().zip(&self.eo).enumerate() {
            let permutation = *ep as usize;
            let orientation = *eo as usize;

            let index = permutation * 2 * 12 + edge_index * 2 + orientation;
            distance += EDGE_DB[index];
        }
        distance as usize
    }
}

pub fn generate_states(cubiecube: CubieCube, solution: &str) -> Vec<FaceCube> {
    let mut current_cube = cubiecube;
    let mut states = Vec::new();

    // Parse the solution string into moves
    let moves: Vec<&str> = solution.split_whitespace().collect();

    // Apply each move and collect the resulting state
    for m in moves {
        let move_cube = CubieCube::from_scramble(m);
        current_cube.multiply(&move_cube);
        states.push(current_cube.to_facelet_cube());
    }

    states
}

pub fn generate_scramble(length: usize) -> String {
    let faces = ["U", "D", "R", "L", "F", "B"];
    let modifiers = ["", "2", "'"]; // Clockwise, 180 degrees, counterclockwise
    let mut rng = rand::thread_rng();

    let mut scramble = Vec::new();
    let mut last_face = "";

    for _ in 0..length {
        let mut face;

        // Ensure no consecutive moves on the same face
        loop {
            face = faces.choose(&mut rng).unwrap();
            if face != &last_face {
                break;
            }
        }

        last_face = face;
        let modifier = modifiers.choose(&mut rng).unwrap();

        scramble.push(format!("{}{}", face, modifier));
    }

    scramble.join(" ")
}

impl PartialEq for CubieCube {
    fn eq(&self, other: &Self) -> bool {
        self.cp == other.cp && self.co == other.co && self.ep == other.ep && self.eo == other.eo
    }
}

impl Hash for CubieCube {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cp.hash(state);
        self.co.hash(state);
        self.ep.hash(state);
        self.eo.hash(state);
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
    fn test_slice() {
        // test get_slice
        let cube = CubieCube::from_scramble("R U L' R2 U' L2");
        assert_eq!(cube.get_slice(), 494);
        let cube = CubieCube::from_scramble("R' D R");
        assert_eq!(cube.get_slice(), 1);

        // test set_slice
        let mut cube = CubieCube::new(None, None, None, None); // Assuming default initializes a solved cube
        cube.set_slice(494);
        assert_eq!(cube.get_slice(), 494);
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_slice(1);
        assert_eq!(cube.get_slice(), 1);
    }

    #[test]
    fn test_slice_sorted() {
        let mut cube = CubieCube::new(None, None, None, None); // Assuming default initializes a solved cube

        // Test set_slice_sorted and get_slice_sorted with various indices
        let test_cases = vec![
            (0, "Solved cube"),      // Solved cube should have index 0
            (11868, "Max index"),    // Maximum index for phase 1
            (24, "Phase 2 example"), // Example for phase 2
            (494, "Random index"),   // Random valid index
        ];

        for (idx, description) in test_cases {
            cube.set_slice_sorted(idx);

            // After setting, the slice index retrieved should match the one set
            let calculated_idx = cube.get_slice_sorted();
            assert_eq!(
                calculated_idx, idx as u16,
                "Failed on test case: {}",
                description
            );
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
            assert_eq!(
                calculated_idx, idx as u16,
                "Failed on test case: {}",
                description
            );
        }
    }

    #[test]
    fn test_d_edges() {
        let mut cube = CubieCube::new(None, None, None, None); // Assuming default initializes a solved cube

        // Test cases: (index, description)
        let test_cases = vec![
            (0, "Solved cube"),          // Solved cube should return 0
            (1656, "Phase 2 example"),   // Example for phase 2
            (1675, "Maximum index"),     // Maximum index for phase 2
            (850, "Random valid index"), // Random valid index
        ];

        for (idx, description) in test_cases {
            // Set the D edges using the index
            cube.set_d_edges(idx);

            // Retrieve the index using get_d_edges
            let calculated_idx = cube.get_d_edges();

            // Assert that the calculated index matches the original
            assert_eq!(
                calculated_idx, idx as u16,
                "Failed on test case: {}",
                description
            );
        }
    }

    #[test]
    fn test_corners() {
        // test get_corners
        let cube = CubieCube::from_scramble("R");
        assert_eq!(cube.get_corners(), 21021);
        let cube = CubieCube::from_scramble("R F");
        assert_eq!(cube.get_corners(), 20924);
        let cube = CubieCube::from_scramble("R R'");
        assert_eq!(cube.get_corners(), 0);
        let cube = CubieCube::from_scramble("R2 B' L U2 D' R' F2 D");
        assert_eq!(cube.get_corners(), 27871);
        // test set_corners
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_corners(21021);
        assert_eq!(cube.get_corners(), 21021);
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_corners(20924);
        assert_eq!(cube.get_corners(), 20924);
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_corners(0);
        assert_eq!(cube.get_corners(), 0);
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_corners(27870);
        assert_eq!(cube.get_corners(), 27870);
    }

    #[test]
    fn test_ud_edges() {
        let cube = CubieCube::from_scramble("R2 U L2");
        assert_eq!(cube.get_ud_edges(), 3834);
        let cube = CubieCube::from_scramble("U F2 D' B2 L2 D2 U'");
        assert_eq!(cube.get_ud_edges(), 15565);
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_ud_edges(21021);
        assert_eq!(cube.get_ud_edges(), 21021);
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_ud_edges(3834);
        assert_eq!(cube.get_ud_edges(), 3834);
        let mut cube = CubieCube::new(None, None, None, None);
        cube.set_ud_edges(15565);
        assert_eq!(cube.get_ud_edges(), 15565);
    }

    #[test]
    fn test_c() {
        let cube = CubieCube::from_scramble("R");
        assert_eq!(cube.c(2, 1), 2);
        assert_eq!(cube.c(3, 1), 3);
        assert_eq!(cube.c(4, 1), 4);
        assert_eq!(cube.c(6, 2), 15);
        assert_eq!(cube.c(7, 2), 21);
        assert_eq!(cube.c(8, 2), 28);
        assert_eq!(cube.c(9, 2), 36);
        assert_eq!(cube.c(10, 2), 45);
    }

    #[test]
    fn test_is_solved() {
        let mut solved = CubieCube::from_scramble("R R'");
        assert!(solved.is_solved());
        solved.multiply(&CubieCube::from_scramble("R"));
        assert!(!solved.is_solved());
    }
}
