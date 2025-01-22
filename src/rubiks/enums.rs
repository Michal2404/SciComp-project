/// Definitions that improve the readability of the code
/// The names of the facelet positions of the cube
///              |************|
///              |*U1**U2**U3*|
///              |************|
///              |*U4**U5**U6*|
///              |************|
///              |*U7**U8**U9*|
///              |************|
/// |************|************|************|************|
/// |*L1**L2**L3*|*F1**F2**F3*|*R1**R2**R3*|*B1**B2**B3*|
/// |************|************|************|************|
/// |*L4**L5**L6*|*F4**F5**F6*|*R4**R5**R6*|*B4**B5**B6*|
/// |************|************|************|************|
/// |*L7**L8**L9*|*F7**F8**F9*|*R7**R8**R9*|*B7**B8**B9*|
/// |************|************|************|************|
///              |************|
///              |*D1**D2**D3*|
///              |************|
///              |*D4**D5**D6*|
///              |************|
///              |*D7**D8**D9*|
///              |************|
/// A cube definition string "UBL..." means for example: In position U1 we have the U-color, in position U2 we have the
/// B-color, in position U3 we have the L color etc. according to the order U1, U2, U3, U4, U5, U6, U7, U8, U9, R1, R2,
/// R3, R4, R5, R6, R7, R8, R9, F1, F2, F3, F4, F5, F6, F7, F8, F9, D1, D2, D3, D4, D5, D6, D7, D8, D9, L1, L2, L3, L4,
/// L5, L6, L7, L8, L9, B1, B2, B3, B4, B5, B6, B7, B8, B9 of the enum constants.
use eframe::egui;

// Enumerations to improve readability of the cube-solving code
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Facelet {
    // The facelet positions of the cube
    U1,
    U2,
    U3,
    U4,
    U5,
    U6,
    U7,
    U8,
    U9,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    // The possible colors of the cube facelets
    U, // Up
    R, // Right
    F, // Front
    D, // Down
    L, // Left
    B, // Back
}

impl Color {
    pub fn to_color32(&self) -> egui::Color32 {
        match self {
            Color::U => egui::Color32::WHITE,
            Color::R => egui::Color32::RED,
            Color::F => egui::Color32::GREEN,
            Color::D => egui::Color32::YELLOW,
            Color::L => egui::Color32::ORANGE,
            Color::B => egui::Color32::BLUE,
        }
    }
    pub fn iter() -> &'static [Color] {
        &[Color::U, Color::R, Color::F, Color::D, Color::L, Color::B]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Ord, Hash)]
pub enum Corner {
    // The corner positions of the cube
    URF = 0, // Up-Right-Front
    UFL = 1, // Up-Front-Left
    ULB = 2, // Up-Left-Back
    UBR = 3, // Up-Back-Right
    DFR = 4, // Down-Front-Right
    DLF = 5, // Down-Left-Front
    DBL = 6, // Down-Back-Left
    DRB = 7, // Down-Right-Back
}

impl Corner {
    /// A constant array of all enum variants
    pub const ALL: [Corner; 8] = [
        Corner::URF,
        Corner::UFL,
        Corner::ULB,
        Corner::UBR,
        Corner::DFR,
        Corner::DLF,
        Corner::DBL,
        Corner::DRB,
    ];
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy, Ord, Hash)]
pub enum Edge {
    // The edge positions of the cube
    UR = 0,  // Up-Right
    UF = 1,  // Up-Front
    UL = 2,  // Up-Left
    UB = 3,  // Up-Back
    DR = 4,  // Down-Right
    DF = 5,  // Down-Front
    DL = 6,  // Down-Left
    DB = 7,  // Down-Back
    FR = 8,  // Front-Right
    FL = 9,  // Front-Left
    BL = 10, // Back-Left
    BR = 11, // Back-Right
    Invalid,
}

impl Edge {
    pub fn iter() -> impl Iterator<Item = Edge> {
        [
            Edge::UR,
            Edge::UF,
            Edge::UL,
            Edge::UB,
            Edge::DR,
            Edge::DF,
            Edge::DL,
            Edge::DB,
            Edge::FR,
            Edge::FL,
            Edge::BL,
            Edge::BR,
        ]
        .iter()
        .copied()
    }
}

impl Edge {
    pub fn from_index(index: usize) -> Edge {
        match index {
            0 => Edge::UR,
            1 => Edge::UF,
            2 => Edge::UL,
            3 => Edge::UB,
            4 => Edge::DR,
            5 => Edge::DF,
            6 => Edge::DL,
            7 => Edge::DB,
            8 => Edge::FR,
            9 => Edge::FL,
            10 => Edge::BL,
            11 => Edge::BR,
            _ => panic!("Invalid edge index!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    // The moves in the face-turn metric
    U1 = 0,
    U2 = 1,
    U3 = 2, // Up face turns
    R1 = 3,
    R2 = 4,
    R3 = 5, // Right face turns
    F1 = 6,
    F2 = 7,
    F3 = 8, // Front face turns
    D1 = 9,
    D2 = 10,
    D3 = 11, // Down face turns
    L1 = 12,
    L2 = 13,
    L3 = 14, // Left face turns
    B1 = 15,
    B2 = 16,
    B3 = 17, // Back face turns
}
impl Move {
    pub fn from_id(id: usize) -> Self {
        match id {
            0 => Move::U1,
            1 => Move::U2,
            2 => Move::U3,
            3 => Move::R1,
            4 => Move::R2,
            5 => Move::R3,
            6 => Move::F1,
            7 => Move::F2,
            8 => Move::F3,
            9 => Move::D1,
            10 => Move::D2,
            11 => Move::D3,
            12 => Move::L1,
            13 => Move::L2,
            14 => Move::L3,
            15 => Move::B1,
            16 => Move::B2,
            17 => Move::B3,
            _ => panic!("Invalid Move ID: {}", id),
        }
    }

    /// Get the number of 90° turns (1, 2, or 3)
    pub fn turns(&self) -> usize {
        match self {
            Move::R1 | Move::U1 | Move::F1 | Move::L1 | Move::B1 | Move::D1 => 1,
            Move::R2 | Move::U2 | Move::F2 | Move::L2 | Move::B2 | Move::D2 => 2,
            Move::R3 | Move::U3 | Move::F3 | Move::L3 | Move::B3 | Move::D3 => 3,
        }
    }

    /// Get the face of the move (e.g., 'R', 'U', etc.)
    pub fn face(&self) -> char {
        match self {
            Move::R1 | Move::R2 | Move::R3 => 'R',
            Move::U1 | Move::U2 | Move::U3 => 'U',
            Move::F1 | Move::F2 | Move::F3 => 'F',
            Move::L1 | Move::L2 | Move::L3 => 'L',
            Move::B1 | Move::B2 | Move::B3 => 'B',
            Move::D1 | Move::D2 | Move::D3 => 'D',
        }
    }

    /// This is optional, but useful if you need to go the other way (variant -> ID).
    pub fn id(&self) -> usize {
        *self as usize
    }
    pub const ALL: [Move; 18] = [
        Move::U1,
        Move::U2,
        Move::U3,
        Move::R1,
        Move::R2,
        Move::R3,
        Move::F1,
        Move::F2,
        Move::F3,
        Move::D1,
        Move::D2,
        Move::D3,
        Move::L1,
        Move::L2,
        Move::L3,
        Move::B1,
        Move::B2,
        Move::B3,
    ];

    pub fn iterator() -> impl Iterator<Item = Move> {
        Move::ALL.iter().copied()
    }
    pub fn name(&self) -> &str {
        match self {
            Move::U1 => "U1",
            Move::U2 => "U2",
            Move::U3 => "U3",
            Move::R1 => "R1",
            Move::R2 => "R2",
            Move::R3 => "R3",
            Move::F1 => "F1",
            Move::F2 => "F2",
            Move::F3 => "F3",
            Move::D1 => "D1",
            Move::D2 => "D2",
            Move::D3 => "D3",
            Move::L1 => "L1",
            Move::L2 => "L2",
            Move::L3 => "L3",
            Move::B1 => "B1",
            Move::B2 => "B2",
            Move::B3 => "B3",
        }
    }
    /// Returns the inverse of a move.
    pub fn invert(self) -> Self {
        use Move::*;
        match self {
            U1 => U3,
            U2 => U2,
            U3 => U1,
            R1 => R3,
            R2 => R2,
            R3 => R1,
            F1 => F3,
            F2 => F2,
            F3 => F1,
            D1 => D3,
            D2 => D2,
            D3 => D1,
            L1 => L3,
            L2 => L2,
            L3 => L1,
            B1 => B3,
            B2 => B2,
            B3 => B1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BasicSymmetry {
    // Basic symmetries of the cube
    RotURF3, // Rotation URF 3 times
    RotF2,   // Rotation Front 2 times
    RotU4,   // Rotation Up 4 times
    MirrLR2, // Mirror Left-Right 2 times
}

/// Create a new move given a face and number of turns
pub fn to_move(face: char, turns: usize) -> Move {
    match (face, turns) {
        ('R', 1) => Move::R1,
        ('R', 2) => Move::R2,
        ('R', 3) => Move::R3,
        ('U', 1) => Move::U1,
        ('U', 2) => Move::U2,
        ('U', 3) => Move::U3,
        ('F', 1) => Move::F1,
        ('F', 2) => Move::F2,
        ('F', 3) => Move::F3,
        ('L', 1) => Move::L1,
        ('L', 2) => Move::L2,
        ('L', 3) => Move::L3,
        ('B', 1) => Move::B1,
        ('B', 2) => Move::B2,
        ('B', 3) => Move::B3,
        ('D', 1) => Move::D1,
        ('D', 2) => Move::D2,
        ('D', 3) => Move::D3,
        _ => panic!("Invalid face or turns"),
    }
}
