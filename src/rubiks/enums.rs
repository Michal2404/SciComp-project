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
use eframe::egui::{Painter, Pos2, Rect, Vec2};

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
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum Corner {
    // The corner positions of the cube
    URF, // Up-Right-Front
    UFL, // Up-Front-Left
    ULB, // Up-Left-Back
    UBR, // Up-Back-Right
    DFR, // Down-Front-Right
    DLF, // Down-Left-Front
    DBL, // Down-Back-Left
    DRB, // Down-Right-Back
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub enum Edge {
    // The edge positions of the cube
    UR, // Up-Right
    UF, // Up-Front
    UL, // Up-Left
    UB, // Up-Back
    DR, // Down-Right
    DF, // Down-Front
    DL, // Down-Left
    DB, // Down-Back
    FR, // Front-Right
    FL, // Front-Left
    BL, // Back-Left
    BR, // Back-Right
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    // The moves in the face-turn metric
    U1,
    U2,
    U3, // Up face turns
    R1,
    R2,
    R3, // Right face turns
    F1,
    F2,
    F3, // Front face turns
    D1,
    D2,
    D3, // Down face turns
    L1,
    L2,
    L3, // Left face turns
    B1,
    B2,
    B3, // Back face turns
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BasicSymmetry {
    // Basic symmetries of the cube
    RotURF3, // Rotation URF 3 times
    RotF2,   // Rotation Front 2 times
    RotU4,   // Rotation Up 4 times
    MirrLR2, // Mirror Left-Right 2 times
}
