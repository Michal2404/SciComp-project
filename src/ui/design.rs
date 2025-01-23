use crate::rubiks::cube::RubiksCube;
// This file creates the design for ui
use crate::ui::app::CubeSettings;
use crate::ui::rotate::MoveQueue;
use crate::cfop::total::cfop_solver;
use crate::ui::pieces::Cubie;

use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_egui::{egui, EguiContexts};

use rand::Rng;
use rand::seq::SliceRandom;


// #[derive(Debug, Resource)]
// pub struct TimekeepingTimer(pub Instant);

// #[derive(Component)]
// struct TextBox;

// #[derive(Component)]
// struct Cursor;


#[derive(Resource, Default, Clone)]
pub struct Scramble {
    input_text: String,
    scramble_content: String,
}

// Scrambling events
#[derive(Debug, Default, Event)]
pub struct ScrambleEvent;
// #[derive(Debug, Default, Event)]
// pub struct GenerateScrambleEvent;

// Resetting event
#[derive(Debug, Default, Event)]
pub struct ResetEvent;

// Solving event
#[derive(Debug, Default, Event)]
pub struct CFOPEvent;


pub fn game_ui(
    mut egui_context: EguiContexts,
    mut cube_settings: ResMut<CubeSettings>,
    mut scramble: ResMut<Scramble>,
    // mut timekeeping_timer: ResMut<TimekeepingTimer>,
    mut scramble_event: EventWriter<ScrambleEvent>,
    mut reset_event: EventWriter<ResetEvent>,
    mut cfop_event: EventWriter<CFOPEvent>,
) {
    egui::Window::new("Rubiks Cube Solver").show(egui_context.ctx_mut(), |ui| {
        egui::Grid::new("ui_grid")
            .num_columns(2)
            .spacing([10.0, 20.0])
            .striped(true)
            .show(ui, |ui| {
                ui.add(egui::Label::new("Rotate Speed"));
                ui.add(egui::Slider::new(
                    &mut cube_settings.rotate_speed,
                    0.1..=10.0,
                ));
                ui.end_row();

                // ui.add(egui::Label::new("Play Mode"));
                // ui.horizontal(|ui| {
                //     ui.selectable_value(
                //         &mut cube_settings.play_mode,
                //         PlayMode::Practice,
                //         "Practice",
                //     );
                //     if ui
                //         .selectable_value(
                //             &mut cube_settings.play_mode,
                //             PlayMode::Timekeeping,
                //             "Timekeeping",
                //         )
                //         .clicked()
                //     {
                //         // 重置计时器
                //         timekeeping_timer.0 = Instant::now();
                //     }
                // });
                // if cube_settings.play_mode == PlayMode::Timekeeping {
                //     ui.add(egui::Label::new(format!(
                //         "{}s",
                //         timekeeping_timer.0.elapsed().as_secs()
                //     )));
                // }
                // ui.end_row();

                // TODO: Think about how to structure scrambles
                // generate scramble
                if ui
                    .add_sized([100.0, 30.0], egui::Button::new("Generate Random Scramble"))
                    .clicked()
                {
                    // first we remove any content from text_input
                    scramble.input_text.clear();
                    // add random scramble
                    scramble.input_text.push_str(generate_random_scramble().as_str());

                    // make temp variable
                    let temp = scramble.input_text.clone();
                    // update the scramble
                    scramble.scramble_content.push_str(&temp)
                }

                // add textbox here
                ui.add(
                    egui::TextEdit::singleline(&mut scramble.input_text)
                    .desired_width(200.0)
                    .hint_text("Type Scramble here ...")
                );
                ui.end_row();
                
                // perform scramble
                if ui
                    .add_sized([100.0, 30.0], egui::Button::new("Scramble"))
                    .clicked()
                {
                    scramble_event.send_default();
                }

                // Reset cube
                if ui
                    .add_sized([100.0, 30.0], egui::Button::new("Reset"))
                    .clicked()
                {
                    reset_event.send_default();
                }
                
                ui.end_row();
                
                // Solve cube
                if ui
                .add_sized([100.0, 30.0], egui::Button::new("Solve"))
                .clicked()
                {
                    // make sure scramble is not empty
                    if !scramble.scramble_content.is_empty(){
                        cfop_event.send_default();
                    }
                }

                ui.end_row();

            });
    });
}

fn generate_random_scramble() -> String {
    /*
    This function generates a random scramble that is 20 characters long
     */
    let possible_moves = vec!["U", "D", "F", "B", "L", "R"];
    let possible_directions = vec!["", "'", "2"];

    // initialize text
    let mut text = String::new();

    // keep a memory of which move was previously played so we dont do consecutive moves that are the same
    let mut excluded = 0;

    let length = 10;

    for i in 0..length {
        // generate random index
        let index = loop {
            let num = rand::thread_rng().gen_range(0..6); // Generate a random number in range [1, 10]
            if num == excluded && i!=0 {
                continue
            }
            else {
                break num; // Exit the loop if the number is not the excluded value
            }
        };
        // pick a random move from the list
        let rotate = possible_moves[index];

        // pick a random direction
        let direction = possible_directions
            .choose(&mut rand::thread_rng())
            .unwrap();

        // combine the 2 strings
        let mut notation = String::new();
        notation.push_str(rotate);
        notation.push_str(direction);

        // push it to text, then add white space
        text.push_str(notation.as_str());
        
        // update the excluded move
        excluded = index;
        
        // if its not the very last move, we add a white space at the end
        if i != length-1{ 
            text.push_str(" ");
        }
    }
    text

    // lastly we update text_input
    // text_input.content = text;
    // println!("{}", text_input.content);
}

pub fn scramble_cube(
    mut events: EventReader<ScrambleEvent>,
    mut move_queue: ResMut<MoveQueue>,
    mut scramble: ResMut<Scramble>,
) {
    /*
    This function takes in the inputted text and scrambles it
     */

    for _ in events.read() {
        // separate based on white space
        let output_list: Vec<String> = vec![scramble.scramble_content.clone()].iter().flat_map(|s| s.split_whitespace()).map(|s| s.to_string()).collect();
        
        // clear text_input
        scramble.input_text.clear();
        // push the notation to the queue
        for notation in output_list.clone(){
            move_queue.0.push_back(notation);
        }
    }
}

pub fn reset_cube(
    mut query: Query<(&mut Transform, &mut Cubie)>,
    mut events: EventReader<ResetEvent>,
    mut scramble: ResMut<Scramble>,
) {
    /*
    This function resets the cube
     */
    for _ in events.read() {
        // Reset the scramble
        scramble.scramble_content.clear();
        // Move the pieces back to its original position
        for (mut transform, mut cubie) in query.iter_mut() {
            transform.translation = cubie.original_position; // Reset to the original position
            // Also reset the rotation
            transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 1.0);
            // reset the left_angle
            cubie.left_angle = 0.0;
        }

    }
}
pub fn solve_cfop(
    mut events: EventReader<CFOPEvent>,
    mut move_queue: ResMut<MoveQueue>,
    mut scramble: ResMut<Scramble>,
) {
    /*
    This function solves the cube using the CFOP method
     */
    for _ in events.read() {
        let mut cube = RubiksCube::new();
        cube.apply_scramble(scramble.scramble_content.as_str());
    
        // solve using cfop
        let output_list = cfop_solver(&scramble.scramble_content.as_str(), cube);

        // push the notation to the queue
        for notation in output_list.clone(){
            move_queue.0.push_back(notation);
        }
        // finally we reset scramble
        scramble.scramble_content.clear();

    }
}