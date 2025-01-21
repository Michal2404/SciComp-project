// This file creates the design for ui
use crate::ui::app::CubeSettings;
use crate::ui::rotate::MoveQueue;
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

#[derive(Resource, Default)]
pub struct InputText {
    content: String,
}

#[derive(Debug, Default, Event)]
pub struct ScrambleEvent;
#[derive(Debug, Default, Event)]
pub struct GenerateScrambleEvent;


pub fn game_ui(
    mut egui_context: EguiContexts,
    mut cube_settings: ResMut<CubeSettings>,
    mut text_input: ResMut<InputText>,
    // mut timekeeping_timer: ResMut<TimekeepingTimer>,
    mut scramble_event: EventWriter<ScrambleEvent>,
    mut generate_scramble_event: EventWriter<GenerateScrambleEvent>,
    // mut reset_event: EventWriter<ResetEvent>,
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

                if ui
                    .add_sized([100.0, 30.0], egui::Button::new("Generate Random Scramble"))
                    .clicked()
                {
                    // generate_scramble_event.send_default();
                    text_input.content.push_str(generate_random_scramble().as_str());
                }

                
                ui.add(
                    egui::TextEdit::singleline(&mut text_input.content)
                    .desired_width(200.0)
                    .hint_text("Type Scramble here ...")
                );
                
                if ui
                    .add_sized([100.0, 30.0], egui::Button::new("Generate Scramble"))
                    .clicked()
                {
                    scramble_event.send_default();
                }
                // if ui
                //     .add_sized([100.0, 30.0], egui::Button::new("Reset"))
                //     .clicked()
                // {
                //     reset_event.send_default();
                // }

                // ui.end_row();
            });
    });
}

fn generate_random_scramble(
    // mut events: EventReader<GenerateScrambleEvent>,
    // mut text_input: ResMut<InputText>,
) -> String {
    /*
    This function generates a random scramble that is 20 characters long
     */
    let possible_moves = vec!["U", "D", "F", "B", "L", "R"];
    let possible_directions = vec!["", "'", "2"];

    // initialize text
    let mut text = String::new();

    // for _ in events.read() {
        for _ in 0..10 {
            // pick a random move from the list
            let rotate = possible_moves
                .choose(&mut rand::thread_rng())
                .unwrap();

            let direction = possible_directions
                .choose(&mut rand::thread_rng())
                .unwrap();

            // combine the 2 strings
            let mut notation = String::new();
            notation.push_str(rotate);
            notation.push_str(direction);

            // push it to text, then add white space
            text.push_str(notation.as_str());
            text.push_str(" ");

        }
    // }
    text
    // lastly we update text_input
    // text_input.content = text;
    // println!("{}", text_input.content);
}

pub fn scramble_cube(
    mut events: EventReader<ScrambleEvent>,
    mut move_queue: ResMut<MoveQueue>,
    text_input: Res<InputText>,
) {
    /*
    This function takes in the inputted text and scrambles it
     */
    // let possible_moves = vec!["U", "D", "F", "B", "L", "R"];
    // let possible_directions = vec!["", "'", "2"];
    let output_list: Vec<String> = vec![text_input.content.clone()].iter().flat_map(|s| s.split_whitespace()).map(|s| s.to_string()).collect();
    for _ in events.read() {
        // for _ in 0..1 {
        // for _ in 0..10 {
            // // pick a random move from the list
            // let rotate = possible_moves
            //     .choose(&mut rand::thread_rng())
            //     .unwrap();

            // let direction = possible_directions
            //     .choose(&mut rand::thread_rng())
            //     .unwrap();

            // // combine the 2 strings
            // let mut notation = String::new();
            // notation.push_str(rotate);
            // notation.push_str(direction);

            // separate based on white space

            for notation in output_list.clone(){
                move_queue.0.push_back(notation);
            }

        // }
    }
}