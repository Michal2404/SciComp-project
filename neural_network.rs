use rand::prelude::*;
use rubiks::rubiks::cube::RubiksCube;
use std::time::Instant;
use std::fs;

// collection of activation functions
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(x: f64) -> f64 {
    x * (1.0 - x)
}

fn relu(x: f64) -> f64 {
    if x > 0.0 { x } else { 0.0 }
}

fn relu_derivative(x: f64) -> f64 {
    if x > 0.0 { 1.0 } else { 0.0 }
}

//Some constants
const SCALE: f64 = 80.0; //For rescaling the values
const EPISODES: usize = 10; //How many episodes should the NN learn
const HIDDEN_NEURONS: usize = 5; // The number of hidden neurons in the NN
const LEARNING_RATE: f64 = 0.01; //The learning rate of the NN
const OUTPUT_BIAS:f64 = 0.0; 



// Generate Test data with states of RubiksCubes and the maximal number of steps to solve them.
fn generate_data(i: i32) -> (Vec<RubiksCube>, Vec<f64>) {
    // Define the possible moves
    let moves = vec![
    String::from("U"),
    String::from("D"),
    String::from("F"),
    String::from("B"),
    String::from("L"),
    String::from("R"),
    ];
    //Define the input and output Vectors for the data
    let mut data_input: Vec<RubiksCube> = Vec::new();
    let mut data_output: Vec<f64> = Vec::new();
    // Create the data
    for _i in 0..i {
        let mut rng = thread_rng(); // Create a random number generator
        let num_moves = rand::thread_rng().gen_range(1..=4); // Random number of moves
        // Shuffle the vector and take the first num_moves elements
        let done_moves: Vec<String> = moves.choose_multiple(&mut rng, num_moves)
                                        .cloned()
                                        .collect();
        let joined_moves = done_moves.join(" "); // Concenated the moves to give an algorithm
        // Create a RubiksCube and scramble
        let mut cube = RubiksCube::new();
        cube.apply_scramble(&joined_moves); // scramble
        data_input.push(cube);
        data_output.push((num_moves as f64)/SCALE); //Normalization
    }
    (data_input, data_output) //return the states and the number of moves done
}

// Get the data from the text files
fn get_data(file_path: &str) -> (Vec<Vec<f64>>, Vec<f64>) {
    // Read the file
    let content = fs::read_to_string(file_path).expect("Error reading the file!");

    // Vectors for the different lines
    let mut lowercase_lines = Vec::new();
    let mut answer_lines = Vec::new();

    // Split the lines and convert them
    for line in content.lines() {
        if line.chars().all(|c| c.is_ascii_lowercase() || c.is_whitespace()) {
            lowercase_lines.push(convert_to_input_vector(&line.to_string())); //Lines, which contain the state of the cube
        } else {
            answer_lines.push(count_commands(&line)/SCALE); //Lines, which contain the moves to solve the state
        }
    }
    (lowercase_lines, answer_lines)
}
// Get the number of how many moves we need to solve the cube
fn count_commands(line: &str) -> f64 {
    let commands: Vec<&str> = line.split_whitespace().collect();
    commands.len() as f64
}
//Convert the lines from the text file to a vector
pub fn convert_to_input_vector(line: &str) -> Vec<f64> {
    let mut input_vector = Vec::with_capacity(6 * 9);

    // Mapping from colors to the numeric values
    let color_map = |color: char| match color {
        'w' => 0.0,
        'y' => 1.0 / 5.0,
        'g' => 2.0 / 5.0,
        'b' => 3.0 / 5.0,
        'r' => 4.0 / 5.0,
        'o' => 1.0,
        _ => panic!("Error color: {}", color),
    };

    for color in line.split_whitespace() {
        let color_value = color_map(color.chars().next().unwrap());
        input_vector.push(color_value);
    }

    input_vector
}

// A function to test the neural network and its accuracy
fn test_nn(prediction: &Vec<f64>, real: &Vec<f64>) -> f64 {
    let mut value: f64 = 0.0;
    for i in 0..real.len() {
        let scaled_prediction = (prediction[i] * SCALE).round(); //Rescaling and rounding
        let scaled_real = real[i] * SCALE; //Rescaling
        
        //println!("Predict: {} and Real: {}", scaled_prediction, scaled_real); //optional priniting of predictions and the correct anwser
        if (scaled_prediction - scaled_real ).abs() == 0.0 { // this is the error measuring
            value += 1.0;
        }
    }
    return value / real.len() as f64 // returns the accuracy of the NN
}

// Define a Neural Network
struct NeuralNetwork {
    input_weights: Vec<f64>, // Weights between input and hidden layer
    hidden_weights: Vec<f64>, // Weights between hidden layer and output
    hidden_bias: Vec<f64>,    // Bias for hidden layer
    output_bias: f64,         // Bias for output layer
    learning_rate: f64,
    hidden_neurons: usize,    // Number of hidden neurons
}

impl NeuralNetwork {
    fn new(hidden_neurons: usize) -> Self {
        let mut rng = rand::thread_rng();
        let input_weights = vec![
                rng.gen_range(-1.0..1.0) / (54.0f64).sqrt(); // Xavier Initialisation
                54 * hidden_neurons
        ];
        let hidden_weights = vec![
                rng.gen_range(-1.0..1.0) / (hidden_neurons as f64).sqrt(); // Xavier Initialisation
                hidden_neurons
        ];

        let hidden_bias = vec![0.0; hidden_neurons]; // Set the hidden bias to 0


        Self {
            input_weights,
            hidden_weights,
            hidden_bias,
            output_bias: OUTPUT_BIAS,
            learning_rate: LEARNING_RATE,
            hidden_neurons,
        }
    }

    fn predict(&self, input: &Vec<f64>) -> f64 {
        // Feedforward pass
        let mut hidden_layer_output = vec![];
        for i in 0..self.hidden_neurons {
            let sum: f64 = input.iter()
                .enumerate()
                .map(|(j, &x)| x * self.input_weights[i * 54 + j]) // Use the input size to calculate weighted sum
                .sum();
            let sum_with_bias = sum + self.hidden_bias[i];
            hidden_layer_output.push(relu(sum_with_bias));
        }

        let output_sum = hidden_layer_output.iter().zip(self.hidden_weights.iter()).map(|(h, w)| h * w).sum::<f64>() + self.output_bias;
        sigmoid(output_sum)
    }
    //Function for training the neural network
    fn train(&mut self, inputs: Vec<Vec<f64>>, outputs: Vec<f64>, epochs: usize) {
        let mut rng = rand::thread_rng();  // Random generator
        for _ in 0..epochs {
            //Iterate over each input vector and its corresponding output
            for (i, cube) in inputs.iter().enumerate() {
                /*Old One
                let input_vector = cube.to_input_vector(); // For the generate Data, where the cubes have to be transformed first.
                */
                let input_vector = cube;
                let mut hidden_layer_output = vec![];
                //Loop thorugh each neuron in the hidden layer
                for j in 0..self.hidden_neurons {
                    let sum: f64 = input_vector.iter()
                        .enumerate()
                        .map(|(k, &x)| x * self.input_weights[j * 54 + k])
                        .sum();
                    let sum_with_bias = sum + self.hidden_bias[j];
                    // Apply the ReLU activation function and store the result
                    hidden_layer_output.push(relu(sum_with_bias));
                }
                // Compute the output sum by combining the hidden layer output and the hidden-to-output weights
                let output_sum = hidden_layer_output.iter().zip(self.hidden_weights.iter()).map(|(h, w)| h * w).sum::<f64>() + self.output_bias;
                // Apply the sigmoid activation function to get the final prediction
                let prediction = sigmoid(output_sum);

                let output_error = outputs[i] - prediction;
                let output_delta = output_error * sigmoid_derivative(prediction); // Calculate the output layer delta (gradient for backpropagation)

                 // Update the hidden-to-output weights, adding noise for regularization
                for j in 0..self.hidden_neurons {
                    let noise: f64 = rng.gen_range(-0.01..0.01);  // Small noise
                    self.hidden_weights[j] += self.learning_rate * output_delta * hidden_layer_output[j] + noise;
                }
                self.output_bias += self.learning_rate * output_delta;

               // Update the input-to-hidden layer weights, with added noise for regularization
                for j in 0..self.hidden_neurons {
                    let hidden_error = output_delta * self.hidden_weights[j] * relu_derivative(hidden_layer_output[j]);
                    // Update each input-to-hidden weight
                    for k in 0..54 {
                        let noise: f64 = rng.gen_range(-0.01..0.01); //Impro
                        self.input_weights[j * 54 + k] += self.learning_rate * hidden_error * input_vector[k] + noise;
                    }
                    // Update the hidden layer bias
                    self.hidden_bias[j] += self.learning_rate * hidden_error;
                }
            }
        }
    }
}



pub fn run() {
    /* OLD data for generated content
    // create trainings data
    let (inputs, outputs) = generate_data(1000);

    // Create test data
    let (test_input, test_ouput) = generate_data(10);
    */

    // New data from files
    let file_path = "../training_data.txt";
    let (inputs, outputs) = get_data(file_path);

    let file_path_test = "../test_data.txt";
    let(test_input, test_ouput) = get_data(file_path_test);

    // create a NN 
    let mut neural_network = NeuralNetwork::new(HIDDEN_NEURONS);

    // Predictions of the NN before traning
    let mut predictions : Vec<f64> = Vec::new();

    for (_i, cube) in test_input.iter().enumerate() {
        /* For generated data only
        // Convert the RubiksCube to a vector of f64 values
        let input_vector = cube.to_input_vector();
        */
        let input_vector = cube;
        predictions.push(neural_network.predict(&input_vector));
    }
    
    println!("Test before training: {}", test_nn(&predictions, &test_ouput));
    
    //Starting time for the training
    let start = Instant::now();
    
    // train the NN 
    neural_network.train(inputs, outputs, EPISODES);
    
    //Ending time for the training
    let duration = start.elapsed().as_secs();

    // Predictions of the NN
    let mut predictions : Vec<f64> = Vec::new();

    for (_i, cube) in test_input.iter().enumerate() {
        /* For generated data only
        // Convert the RubiksCube to a vector of f64 values
        let input_vector = cube.to_input_vector();
        */
        let input_vector = cube;
        predictions.push(neural_network.predict(&input_vector));
    }
    
    println!("Test after training: {}", test_nn(&predictions, &test_ouput));
    println!("Elapsed time: {:?} seconds", duration);
    println!("The trained NN:\nEpisodes: {}\nLearning-rate: {}\nOutputbias: {}\nHidden neurons: {}", EPISODES, LEARNING_RATE, OUTPUT_BIAS, HIDDEN_NEURONS);
}
