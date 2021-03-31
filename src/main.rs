use std::io;

pub mod rover;
use rover::{
    parse_rover_to_deploy, parse_user_plateau, print_final_rover_positions, simulate_rover_move,
};

fn main() {
    println!("\nWelcome to NASA's Mars Rover Simulator\n");
    println!("Commands are entered line by line.");
    println!("After typing your commands, hit the enter/return key to input the command into the simulator.");
    println!("To quit, press the 'ctrl+c' keyboard combination.");
    println!(
        "Press 'd' then the return/enter key when you're ready to simulate the commands entered."
    );
    println!("Please enter your commands to begin...\n");

    let mut user_input_lines: Vec<String> = Vec::new();
    let user_input_line_delimiter = "d";

    // Start loop to receive user commands of unknown number
    loop {
        let mut terminal_line = String::new();

        io::stdin()
            .read_line(&mut terminal_line)
            .expect("Failed to read line");

        terminal_line = terminal_line.trim().to_string();
        println!("You entered: '{}'...", terminal_line);
        // Check if user has finished entering their commands to break loop
        if terminal_line == user_input_line_delimiter {
            break;
        }

        user_input_lines.push(terminal_line);
    }

    if !user_input_lines.is_empty() {
        user_input_lines.reverse(); // Reverse to treat as a stack

        let ur_plateau = match parse_user_plateau(user_input_lines.pop().unwrap()) {
            Ok(plateau) => plateau,
            Err(err) => {
                println!("Error: {:?}. Please check plateau coordinates.", err);
                std::process::exit(1)
            }
        };

        let input_command = match parse_rover_to_deploy(ur_plateau, user_input_lines) {
            Ok(input_command) => input_command,
            Err(err) => {
                println!("Error: {:?}. Please check your rover command(s).", err);
                std::process::exit(1)
            }
        };

        match simulate_rover_move(input_command) {
            Ok(rover_positions) => print_final_rover_positions(rover_positions),
            Err(err) => println!("Error: {:?}. Please check your rover command(s).", err),
        }
    }
}
