// Defining the cube on the Facelet Level with positions of the stickers.

use super::cubie::CubieCube;
use super::defs::{CORNER_COLOR, CORNER_FACELET, EDGE_COLOR, EDGE_FACELET};
use super::enums::{Color, Corner as Co, Edge as Ed};
use crate::rubiks_two_phase::cubie::FromUsize;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FaceCube {
    pub f: [Color; 54],
}

impl FaceCube {
    pub fn new() -> Self {
        let mut f = [Color::U; 54];

        // Update specific ranges using iterators
        f.iter_mut().skip(9).take(9).for_each(|c| *c = Color::R);
        f.iter_mut().skip(18).take(9).for_each(|c| *c = Color::F);
        f.iter_mut().skip(27).take(9).for_each(|c| *c = Color::D);
        f.iter_mut().skip(36).take(9).for_each(|c| *c = Color::L);
        f.iter_mut().skip(45).take(9).for_each(|c| *c = Color::B);

        FaceCube { f }
    }

    pub fn from_string(&mut self, s: &str) -> Result<(), String> {
        if s.len() != 54 {
            return Err(format!(
                "Error: Cube definition string contains {} facelets instead of 54.",
                s.len()
            ));
        }

        let mut cnt = [0; 6];
        for (i, c) in s.chars().enumerate() {
            self.f[i] = match c {
                'U' => {
                    cnt[Color::U as usize] += 1;
                    Color::U
                }
                'R' => {
                    cnt[Color::R as usize] += 1;
                    Color::R
                }
                'F' => {
                    cnt[Color::F as usize] += 1;
                    Color::F
                }
                'D' => {
                    cnt[Color::D as usize] += 1;
                    Color::D
                }
                'L' => {
                    cnt[Color::L as usize] += 1;
                    Color::L
                }
                'B' => {
                    cnt[Color::B as usize] += 1;
                    Color::B
                }
                _ => return Err(format!("Error: Invalid character '{}' in cube string.", c)),
            };
        }

        if cnt.iter().all(|&x| x == 9) {
            Ok(())
        } else {
            Err(
                "Error: Cube definition string does not contain exactly 9 facelets of each color."
                    .to_string(),
            )
        }
    }

    pub fn to_cubie_cube(&self) -> CubieCube {
        let mut cc = CubieCube::new(None, None, None, None);
        for (i, &fac) in CORNER_FACELET.iter().enumerate() {
            let mut ori = 0;
            for (k, item) in fac.iter().enumerate() {
                if self.f[*item as usize] == Color::U || self.f[*item as usize] == Color::D {
                    ori = k;
                    break;
                }
            }

            let col1 = self.f[fac[(ori + 1) % 3] as usize];
            let col2 = self.f[fac[(ori + 2) % 3] as usize];

            for (j, &col) in CORNER_COLOR.iter().enumerate() {
                if col1 == col[1] && col2 == col[2] {
                    cc.cp[i] = Co::from_usize(j).unwrap();
                    cc.co[i] = ori as u8;
                    break;
                }
            }
        }

        for (i, &ef) in EDGE_FACELET.iter().enumerate() {
            for (j, &ec) in EDGE_COLOR.iter().enumerate() {
                if self.f[ef[0] as usize] == ec[0] && self.f[ef[1] as usize] == ec[1] {
                    cc.ep[i] = Ed::from_usize(j).unwrap();
                    cc.eo[i] = 0;
                    break;
                }
                if self.f[ef[0] as usize] == ec[1] && self.f[ef[1] as usize] == ec[0] {
                    cc.ep[i] = Ed::from_usize(j).unwrap();
                    cc.eo[i] = 1;
                    break;
                }
            }
        }

        cc
    }
}

impl fmt::Display for FaceCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cube_string: String = self
            .f
            .iter()
            .map(|&color| match color {
                Color::U => 'U',
                Color::R => 'R',
                Color::F => 'F',
                Color::D => 'D',
                Color::L => 'L',
                Color::B => 'B',
            })
            .collect();
        write!(f, "{}", cube_string)
    }
}

impl Default for FaceCube {
    fn default() -> Self {
        Self::new()
    }
}
