use std::time::Instant;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy_egui::egui::{Frame, Ui};


// This file creates the design for ui
use crate::rubiks::cube::RubiksCube;
use crate::ui::app::CubeSettings;
use crate::ui::rotate::MoveQueue;
use crate::cfop::total::cfop_solver;
use crate::a_star::a_star::a_star_solver;
use crate::ui::pieces::Cubie;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use rand::Rng;
use rand::seq::SliceRandom;

use futures_lite::future;

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
    solve_content: String,
    index: usize,
}

// UI information resource for solver
struct Solver {
    name: String,
    checked: bool,
}

#[derive(Resource)]
pub struct SolverInformation {
    solvers: Vec<Solver>,
}

impl Default for SolverInformation {
    fn default() -> Self {
        Self {
            solvers: vec![
                Solver {
                    name: "CFOP Solver".to_string(),
                    checked: false,
                },
                Solver {
                    name: "A star Solver".to_string(),
                    checked: false,
                },
                // Add more solvers as needed
            ],
        }
    }
}

impl SolverInformation {
    fn uncheck_all_except(&mut self, solver_name: &str) {
        for solver in self.solvers.iter_mut() {
            solver.checked = solver.name == solver_name.to_string();
        }
    }
}

// UI information
#[derive(Debug, Default, Resource)]
pub struct UiInformation {
    pub step_by_step: bool,
}

// time keeping resource
#[derive(Debug, Resource)]
pub struct TimekeepingTimer{
    pub time: Instant,
    pub running: bool,
    pub last_time: u128,
}

impl Default for TimekeepingTimer {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            running: false,
            last_time: 0,
        }
    }
}

// Define a resource to store the async task
#[derive(Resource, Default)]
pub struct SolveTask(pub Option<Task<Vec<String>>>);

#[derive(Debug, Default, Event)]
pub struct ScrambleEvent;

// Resetting event
#[derive(Debug, Default, Event)]
pub struct ResetEvent;

// Solving event
#[derive(Debug, Default, Event)]
pub struct SolveEvent;


pub fn game_ui(
    mut egui_context: EguiContexts,
    mut cube_settings: ResMut<CubeSettings>,
    mut scramble: ResMut<Scramble>,
    mut solve_data: ResMut<SolveData>,
    mut timer: ResMut<TimekeepingTimer>,
    mut solver_information: ResMut<SolverInformation>,
    mut ui_information: ResMut<UiInformation>,
    mut move_queue: ResMut<MoveQueue>,
    mut scramble_event: EventWriter<ScrambleEvent>,
    mut reset_event: EventWriter<ResetEvent>,
    mut solve_event: EventWriter<SolveEvent>,
) {
    egui::Window::new("Rubiks Cube Solver").show(egui_context.ctx_mut(), |ui| {
        // egui::Grid::new("ui_grid")
        //     // .num_columns(2)
        //     .spacing([10.0, 20.0])
        //     .striped(true)
        //     .show(ui, |ui| {
            // Flex::horizontal().show(ui, |flex| {
        // egui::Grid::new("ui_grid").vertical(|mut strip| {

        //  define column widths
        let col1_width = 100.0;
        let col2_width = 100.0;
        let col_spacing = 30.0;
        let row_spacing = 20.0;


        // adjust scramble length
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(col_spacing);
                ui.add_sized([col1_width, 20.0],egui::Label::new("Scramble Length"));
                ui.add_space(col_spacing);
            });
            ui.add_space(col_spacing);
            
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_sized([col2_width, 20.0], egui::Slider::new(
                    &mut cube_settings.num_scramble_moves,
                    1..=20,
                ));
            });
        });
        ui.end_row();
        ui.add_space(row_spacing);
        
        // generate scramble
        ui.horizontal(|ui| {
            // generate scramble
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                if ui
                .add_sized([col1_width, 20.0], egui::Button::new("Generate Random Scramble"))
                .clicked()
                {
                    // first we remove any content from text_input
                    scramble.input_text.clear();
                    // add random scramble
                    scramble.input_text.push_str(generate_random_scramble(&cube_settings).as_str());
                }
            });
            ui.add_space(col_spacing);
            // add textbox here
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_sized([150.0, 20.0],
                    egui::TextEdit::singleline(&mut scramble.input_text)
                    .desired_width(200.0)
                    .hint_text("Type Scramble here ...")
                );
            });
        });
        ui.end_row();
        ui.add_space(row_spacing);
        
        // scramble cube and reset cube buttons
        ui.horizontal(|ui| {
            // perform scramble
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(col_spacing);
                // can only click scramble if there is a scramble
                if ui.add_sized([col1_width, 30.0], egui::Button::new("Scramble")).clicked() && (solve_data.index == 0 || solve_data.index == solve_data.solve_sequence.len()) {
                    scramble_event.send_default();
                }
                ui.add_space(col_spacing);
            });
            // Reset cube
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(col_spacing);
                if ui.add_sized([col2_width, 30.0], egui::Button::new("Reset")).clicked()
                {
                    reset_event.send_default();
                }
                ui.add_space(col_spacing);
            });
        });
        
        
        ui.end_row();
        ui.add_space(row_spacing);
        
        // Display current scramble
        ui.horizontal(|ui| {
                ui.add(egui::Label::new(egui::RichText::new(scramble.scramble_content.clone())
                .font(egui::FontId::proportional(20.0)))
                .wrap()
            );
        });
        ui.end_row();
        ui.add_space(row_spacing);

        // Solver
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(col_spacing);
                ui.add(egui::Label::new("Solver"));
                ui.add_space(col_spacing);
            });
        });

        // Enable or disable CFOP solver
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(col_spacing);
                // Enable or disable CFOP solver
                let cfop_solver = solver_information.solvers.iter_mut().find(|s| s.name == "CFOP Solver").unwrap();
                if ui.add(egui::Checkbox::new(&mut cfop_solver.checked, "CFOP Solver")).clicked() {
                    solver_information.uncheck_all_except("CFOP Solver");
                }

                ui.add_space(col_spacing);

                // Enable or disable A star solver
                let astar_solver = solver_information.solvers.iter_mut().find(|s| s.name == "A star Solver").unwrap();
                if ui.add(egui::Checkbox::new(&mut astar_solver.checked, "A star Solver")).clicked() {
                    solver_information.uncheck_all_except("A star Solver");
                }

                ui.add_space(col_spacing);
            });
            ui.add_space(col_spacing);
        });
        ui.end_row();
        ui.add_space(row_spacing);
        
        // add if we want to solve step by step
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                // Enable or disable moving back and fourth
                ui.add_space(col_spacing);
                ui.add(egui::Checkbox::new(&mut ui_information.step_by_step, "Step by Step"));
                ui.add_space(col_spacing);
            });
            ui.add_space(col_spacing);

            // we will make forward and backward button if step by step is enabled
            if ui_information.step_by_step {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(col_spacing);
                        if ui.add(egui::Button::new("Previous")).clicked() && solve_data.index > 0 {
                            // look at previous index
                            solve_data.index -= 1;
                            // we look at the previous moves and determine its reverse
                            let reverse_move = if solve_data.solve_sequence[solve_data.index].ends_with('\'') {
                                // Remove the trailing `'` if it exists
                                solve_data.solve_sequence[solve_data.index].trim_end_matches('\'').to_string()
                            } else if solve_data.solve_sequence[solve_data.index].ends_with('2') {
                                solve_data.solve_sequence[solve_data.index].clone()
                            } else {
                                // Add `'` if it doesn't exist
                                format!("{}'", solve_data.solve_sequence[solve_data.index].clone())
                            };
                            
                            // now push this to move_queue
                            move_queue.0.push_back(reverse_move);
                        }
                        ui.add_space(col_spacing);
                        if ui.add(egui::Button::new("Next")).clicked() && !solve_data.solve_sequence.is_empty(){
                            // if solve_data.index is at the solution length, we continue
                            if solve_data.index < solve_data.solve_sequence.len() - 1 {
                                // we push the first value to the back
                                move_queue.0.push_back(solve_data.solve_sequence[solve_data.index].clone());
                                // look at next index
                                solve_data.index += 1;
                            }
                        }
                        ui.add_space(col_spacing);
                    });
                    ui.add_space(col_spacing);
                });
            }
        });
        ui.end_row();
        ui.add_space(row_spacing);
        
        // Solve cube
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(col_spacing);
                // Solve cube
                if ui.add_sized([100.0, 30.0], egui::Button::new("Solve")).clicked()
                {
                    // make sure scramble is not empty and also one solver is selected
                    if !scramble.scramble_content.is_empty() && solver_information.solvers.iter().any(|s| s.checked) {
                        timer.time = Instant::now();
                        timer.running = true;
                        solve_event.send_default();
                        solve_data.index = 0;
                        solve_data.solve_sequence.clear();
                    }
                }
            });
            ui.add_space(col_spacing);
        });
        ui.end_row();
        ui.add_space(row_spacing);

        // Display current move
        // ui.horizontal_wrapped(|ui| {
        //     // ui.add_sized()
        //         ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        //             ui.add_space(col_spacing);
        //             for (i, move_) in solve_data.solve_sequence.iter().enumerate() {
        //                 let text = if i as isize == solve_data.index as isize - 1 {
        //                     egui::RichText::new(move_.clone()).color(egui::Color32::GREEN)
        //                 } else {
        //                     egui::RichText::new(move_.clone())
        //                 };
        //                 // ui.add_sized([100.0, 30.0],egui::Label::new(text).wrap());
        //                 ui.add(egui::Label::new(text).wrap());
        //                 // let label = ui.label(text);

        //                 // println!("{:?}", label.rect.width());
        //                 // println!("{:?}", label.rect.);
        //                 // ui.add_space(2.0);
        //             }
        //             ui.add_space(col_spacing);
        //         });
        //         ui.add_space(col_spacing);
        //     // ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        //     //     ui.add_space(col_spacing);
        //     //     for (i, move_) in solve_data.solve_sequence.iter().enumerate() {
        //     //         let text = if i as isize == solve_data.index as isize - 1 {
        //     //             egui::RichText::new(move_.clone()).color(egui::Color32::GREEN)
        //     //         } else {
        //     //             egui::RichText::new(move_.clone())
        //     //         };
        //     //         // ui.add(egui::Label::new(text).wrap());
        //     //         let label = ui.label(text);

        //     //         // println!("{:?}", label.rect.width());
        //     //         // println!("{:?}", label.rect.);
        //     //         ui.add_space(2.0);
        //     //     }
        //     //     ui.add_space(col_spacing);
        //     // });
        //     ui.add_space(col_spacing);
        // });


        // Display current solve sequence
        ui.horizontal(|ui| {
            ui.add(egui::Label::new(egui::RichText::new(solve_data.solve_content.clone())
                .font(egui::FontId::proportional(20.0)))
                .wrap()
            );
        });
        ui.end_row();
        ui.add_space(row_spacing);
        
        // Time taken to solve the rubiks cube and number of moves
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add(egui::Label::new("Time Taken"));
            });
            // check if timer is running
            let time_taken = match timer.running {
                true => timer.time.elapsed().as_millis(),
                false => timer.last_time,
            };
            
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add(egui::Label::new(format!("{:?} ms", time_taken)));
            });
        });
        ui.end_row();
        ui.add_space(row_spacing);
        
        // Total move length
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add(egui::Label::new("Move Length"));
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add(egui::Label::new(solve_data.solve_sequence.len().to_string()));
            });
        });
        ui.end_row();
    
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
    ui_information: ResMut<UiInformation>,
    mut scramble: ResMut<Scramble>,
    mut timer: ResMut<TimekeepingTimer>,
) {
    /*
    This function polls the async task and handles the result
    */
    // if the task is not none
    if let Some(task) = solve_task.0.as_mut() {
        // get the output list and duration
        if let Some(output_list) = future::block_on(future::poll_once(task)) {
            // add the sequence to solve data
            solve_data.solve_content = output_list.clone().join(" ");

            // if step by step is enabled, we dont perfrom this
            if !ui_information.step_by_step {
                // Push the notation to the queue
                for notation in output_list.clone() {
                    move_queue.0.push_back(notation);
                }
                
                // Finally we reset scramble
                scramble.scramble_content.clear();

            }
            
            // Add the moves to solve_data
            solve_data.solve_sequence = output_list.clone();

            // Clear the task
            solve_task.0 = None;
            // stop the timer
            timer.running = false;
            timer.last_time = timer.time.elapsed().as_millis();
        }
    }
}

pub fn solve_cube(
    mut events: EventReader<SolveEvent>,
    scramble: ResMut<Scramble>,
    mut solve_task: ResMut<SolveTask>,
    solver_information: Res<SolverInformation>,
) {
    /*
    This function solves the cube using the selected solver
    */
    for _ in events.read() {
        // initialize the scramble variable
        let scramble_sequence = scramble.scramble_content.clone();
        let mut cube = RubiksCube::new();
        cube.apply_scramble(scramble_sequence.as_str());
        
        // Clone necessary data for the async task
        let scramble_sequence_clone = scramble_sequence.clone();
        
        // Spawn a task on Bevy's thread pool
        let task_pool = AsyncComputeTaskPool::get();
        if let Some(solver) = solver_information.solvers.iter().find(|s| s.checked) {
            match solver.name.as_str() {
                "CFOP Solver" => {
                    let result = task_pool.spawn(async move {
                        // solve_data_clone.time_taken.tick(time_clone.delta());
                        cfop_solver(&scramble_sequence_clone.as_str(), cube)
                    });
                    // Store the task in the resource
                    solve_task.0 = Some(result);
                }
                "A star Solver" => {
                    let result = task_pool.spawn(async move {
                        // solve_data_clone.time_taken.tick(time_clone.delta());
                        a_star_solver(&scramble_sequence_clone.as_str(), &mut cube)
                    });
                    // Store the task in the resource
                    solve_task.0 = Some(result);
                }
                _ => {}
            }
        }
    }
}