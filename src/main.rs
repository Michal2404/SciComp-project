#![allow(clippy::needless_range_loop)]
mod rubiks;
mod cfop;
mod ui;
mod helper;
mod rubiks_two_phase;

use ui::app::*;
fn main() {
    // run the visualization
    run_visualization(true);
}