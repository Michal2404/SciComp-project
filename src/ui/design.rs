use std::time::Duration;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::time::Stopwatch;


// This file creates the design for ui
use crate::rubiks::cube::RubiksCube;
use crate::ui::app::CubeSettings;
use crate::ui::rotate::MoveQueue;
use crate::cfop::total::cfop_solver;
use crate::a_star::a_star::a_star_solver;
use crate::a_star::parallel_a_star::parallel_a_star_solver;
use crate::ui::pieces::Cubie;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use rand::Rng;
use rand::seq::SliceRandom;

use futures_lite::future;


// #[derive(Debug, Resource)]
// pub struct TimekeepingTimer(pub Instant);

// Interacting with egui
#[derive(Debug, Default, Event)]
pub struct EguiInteraction;

// Scrambling resource
#[derive(Resource, Default, Clone)]
pub struct Scramble {
    input_text: String,
    scramble_content: String,
}

// Solving resource
#[derive(Resource, Default, Clone)]
pub struct SolveData {
    solve_sequence: Vec<String>,
    time_taken: Stopwatch,
}

// Define a resource to store the async task
#[derive(Resource, Default)]
pub struct SolveTask(pub Option<Task<Vec<String>>>);
// pub struct SolveTask(pub Option<Task<(Vec<String>, Stopwatch)>>);

#[derive(Debug, Default, Event)]
pub struct ScrambleEvent;

// Resetting event
#[derive(Debug, Default, Event)]
pub struct ResetEvent;

// Solving event
#[derive(Debug, Default, Event)]
pub struct CFOPEvent;
#[derive(Debug, Default, Event)]
pub struct ASTAREvent;


pub fn game_ui(
    mut egui_context: EguiContexts,
    mut cube_settings: ResMut<CubeSettings>,
    mut scramble: ResMut<Scramble>,
    solve_data: ResMut<SolveData>,
    // mut timekeeping_timer: ResMut<TimekeepingTimer>,
    mut scramble_event: EventWriter<ScrambleEvent>,
    mut reset_event: EventWriter<ResetEvent>,
    mut cfop_event: EventWriter<CFOPEvent>,
    mut astar_event: EventWriter<ASTAREvent>,
) {
    egui::Window::new("Rubiks Cube Solver").show(egui_context.ctx_mut(), |ui| {
        egui::Grid::new("ui_grid")
            .num_columns(2)
            .spacing([10.0, 20.0])
            .striped(true)
            .show(ui, |ui| {
                ui.add(egui::Label::new("Scramble Length"));
                ui.add(egui::Slider::new(
                    &mut cube_settings.num_scramble_moves,
                    1..=20,
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
                    scramble.input_text.push_str(generate_random_scramble(&cube_settings).as_str());
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
                
                // Display current scramble
                ui.add(egui::Label::new(scramble.scramble_content.clone()));
                ui.end_row();
                
                // Solve cube CFOP
                if ui
                .add_sized([100.0, 30.0], egui::Button::new("CFOP Solve"))
                .clicked()
                {
                    // make sure scramble is not empty
                    if !scramble.scramble_content.is_empty(){
                        cfop_event.send_default();
                    }
                }
                // Solve cube A star
                if ui
                .add_sized([100.0, 30.0], egui::Button::new("A star Solve"))
                .clicked()
                {
                    // make sure scramble is not empty
                    if !scramble.scramble_content.is_empty(){
                        astar_event.send_default();
                    }
                }
                ui.end_row();

                // Time taken to solve the rubiks cube and number of moves
                ui.add(egui::Label::new("Time Taken"));
                ui.add(egui::Label::new(format!("{:?} ms", solve_data.time_taken.elapsed().as_millis())));
                ui.end_row();
                
                // Total move length
                ui.add(egui::Label::new("Move Length"));
                ui.add(egui::Label::new(solve_data.solve_sequence.len().to_string()));
                ui.end_row();
                
            });
    });
}

fn generate_random_scramble(
    cube_settings: &CubeSettings,
) -> String {
    /*
    This function generates a random scramble that is 20 characters long
     */
    let possible_moves = vec!["U", "D", "F", "B", "L", "R"];
    let possible_directions = vec!["", "'", "2"];

    // initialize text
    let mut text = String::new();

    // keep a memory of which move was previously played so we dont do consecutive moves that are the same
    let mut excluded = 0;

    for i in 0..cube_settings.num_scramble_moves {
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
        if i != cube_settings.num_scramble_moves-1{ 
            text.push_str(" ");
        }
    }
    text

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
        // clear any scramble that was done previously
        scramble.scramble_content.clear();

        // make temp variable
        let temp = scramble.input_text.clone();
        // update the scramble
        scramble.scramble_content.push_str(&temp);

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
    mut move_queue: ResMut<MoveQueue>,
) {
    /*
    This function resets the cube
     */
    for _ in events.read() {
        // Reset the scramble
        scramble.scramble_content.clear();
        // Also reset any moves currently being done
        move_queue.0.clear();
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

pub fn poll_solve_task(
    mut solve_task: ResMut<SolveTask>,
    mut move_queue: ResMut<MoveQueue>,
    mut solve_data: ResMut<SolveData>,
    mut scramble: ResMut<Scramble>,
) {
    /*
    This function polls the async task and handles the result
    */
    // if the task is not none
    if let Some(task) = solve_task.0.as_mut() {
        // get the output list and duration
        // if let Some((output_list, duration)) = future::block_on(future::poll_once(task)) {
        if let Some(output_list) = future::block_on(future::poll_once(task)) {
            // Push the notation to the queue
            for notation in output_list.clone() {
                move_queue.0.push_back(notation);
            }
            
            // Add the moves and duration to solve_data
            solve_data.solve_sequence = output_list;
            // solve_data.time_taken = duration;
            
            // Finally we reset scramble
            scramble.scramble_content.clear();
            
            // Clear the task
            solve_task.0 = None;
            // stop the timer
            solve_data.time_taken.pause();
        }
    }
}

pub fn solve_cfop(
    mut events: EventReader<CFOPEvent>,
    scramble: ResMut<Scramble>,
    mut solve_task: ResMut<SolveTask>,
    mut solve_data: ResMut<SolveData>,
    time: Res<Time>,
) {
    /*
    This function solves the cube using the CFOP method
    */
    for _ in events.read() {
        // initialize the scramble variable
        let scramble_sequence = scramble.scramble_content.clone();
        let mut cube = RubiksCube::new();
        cube.apply_scramble(scramble_sequence.as_str());
        
        // initialize timer and reset it
        // solve_data.time_taken = Stopwatch::new();
        solve_data.time_taken.reset();
        // solve_data.time_taken.tick(time.delta());
        
        // solve using cfop
        // Clone necessary data for the async task
        let scramble_sequence_clone = scramble_sequence.clone();
        let mut solve_data_clone = solve_data.clone();
        let time_clone = time.clone();
        
        // solve using cfop
        // Spawn a task on Bevy's thread pool
        let task_pool = AsyncComputeTaskPool::get();
        let result = task_pool.spawn(async move {
            solve_data_clone.time_taken.tick(time_clone.delta());
            cfop_solver(&scramble_sequence_clone.as_str(), cube)
        });
        // // Spawn a task on Bevy's thread pool
        // let task_pool = AsyncComputeTaskPool::get();
        // let result = task_pool.spawn(async move {
        //     solve_data.time_taken.tick(time.delta());
        //     cfop_solver(&scramble_sequence.as_str(), cube)
        //     // let time_taken = solve_data.time_taken.tick(time_clone.delta());
        //     // let moves = cfop_solver(&scramble_sequence.as_str(), cube);
        //     // (moves, time_taken.clone())
        // });
        
        // Store the task in the resource
        solve_task.0 = Some(result);
        
    }
}
pub fn solve_astar(
    mut events: EventReader<ASTAREvent>,
    scramble: ResMut<Scramble>,
    mut solve_task: ResMut<SolveTask>,
    mut solve_data: ResMut<SolveData>,
    time: Res<Time>,
) {
    /*
    This function solves the cube using the CFOP method
    */
    for _ in events.read() {
        // initialize the scramble variable
        let scramble_sequence = scramble.scramble_content.clone();
        let mut cube = RubiksCube::new();
        cube.apply_scramble(scramble_sequence.as_str());
        
        // initialize timer
        solve_data.time_taken.tick(time.delta());

        
        // solve using cfop
        // Clone necessary data for the async task
        let scramble_sequence_clone = scramble_sequence.clone();
        let mut solve_data_clone = solve_data.clone();
        let time_clone = time.clone();
        
        // solve using cfop
        // Spawn a task on Bevy's thread pool
        let task_pool = AsyncComputeTaskPool::get();
        let result = task_pool.spawn(async move {
            solve_data_clone.time_taken.tick(time_clone.delta());
            a_star_solver(&scramble_sequence_clone.as_str(), &mut cube)
        });
        // // Spawn a task on Bevy's thread pool
        // let task_pool = AsyncComputeTaskPool::get();
        // let result = task_pool.spawn(async move {
        //         a_star_solver(&scramble_sequence.as_str(), &mut cube)
        // });

        // Store the task in the resource
        solve_task.0 = Some(result);
        
    }
}