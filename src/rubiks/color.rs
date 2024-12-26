use eframe::egui::Color32;

// Defining colors of the stickers
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Color {
    W, // White
    Y, // Yellow
    G, // Green
    B, // Blue
    R, // Red
    O, // Orange
}

impl Color {
    pub fn to_color32(&self) -> Color32 {
        match self {
            Color::W => Color32::WHITE,
            Color::Y => Color32::from_rgb(255, 255, 0),
            Color::G => Color32::from_rgb(0, 255, 0),
            Color::B => Color32::from_rgb(0, 0, 255),
            Color::R => Color32::from_rgb(255, 0, 0),
            Color::O => Color32::from_rgb(255, 165, 0),
        }
    }
}
