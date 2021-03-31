use std::io;

pub mod rover;
use rover::{move_rover, parse_rover_to_deploy, parse_user_plateau, print_final_rover_positions};

fn main() {
    println!("\nWelcome to NASA's Mars Rover Simulator\n");
    println!("Commands are entered line by line.");
    println!("After typing your commands, hit the enter/return key to input the command into the simulator.");
    println!("To quit, press the 'ctrl+c' keyboard combination.");
    println!(
        "Press 'd' then the return/enter key when you're ready to simulate the commands entered."
    );
    println!("Please enter your commands to begin...\n");

    let mut user_input: Vec<String> = Vec::new();

    loop {
        let mut terminal_line = String::new();

        io::stdin()
            .read_line(&mut terminal_line)
            .expect("Failed to read line");

        terminal_line = terminal_line.trim().to_string();
        println!("You entered: '{}'...", terminal_line);
        if terminal_line == *"d" {
            break;
        }
        user_input.push(terminal_line);
    }

    if !user_input.is_empty() {
        user_input.reverse();

        let ur_plateau = match parse_user_plateau(user_input.pop().unwrap()) {
            Ok(plateau) => plateau,
            Err(err) => {
                println!("Error: {:?}. Please check plateau coordinates.", err);
                std::process::exit(1)
            }
        };

        let input_command = match parse_rover_to_deploy(ur_plateau, user_input) {
            Ok(input_command) => input_command,
            Err(err) => {
                println!("Error: {:?}. Please check your rover command(s).", err);
                std::process::exit(1)
            }
        };

        match move_rover(input_command) {
            Ok(rover_positions) => print_final_rover_positions(rover_positions),
            Err(err) => println!("Error: {:?}. Please check your rover command(s).", err),
        }
    }
}
