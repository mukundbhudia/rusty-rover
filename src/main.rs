use std::io;

pub mod rover;
use rover::{
    move_rover, parse_rover_commands, parse_user_plateau, print_final_rover_positions, InputCommand,
};

fn main() {
    println!("\nWelcome to NASA's Mars Rover Simulator\n");
    println!("Commands are entered line by line. After typing your commands you can hit the enter/return key to input the command.");
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
            Err(_) => {
                println!("Error: Please check plateau coordinates.");
                // TODO: a better way to exit?
                std::process::exit(1)
            }
        };

        let mut input_command = InputCommand {
            ur_plateau,
            rovers_to_deploy: Vec::new(),
        };

        if !user_input.is_empty() && user_input.len() % 2 == 0 {
            for command in user_input.chunks(2) {
                let rover_start_position = command[1]
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<Vec<_>>();

                if let Ok(rover_start) = parse_rover_commands(rover_start_position) {
                    let rover_start_position = (rover_start, command[0].to_string());
                    input_command.rovers_to_deploy.push(rover_start_position);
                } else {
                    println!(
                        "Error: Bad rover start position. Please check coordinates and heading."
                    );
                }
            }
            if !input_command.rovers_to_deploy.is_empty() {
                input_command.rovers_to_deploy.reverse();

                match move_rover(input_command) {
                    Ok(rover_positions) => print_final_rover_positions(rover_positions),
                    Err(err) => println!("The rover had an error: {:?}. Please check one or more of your rover commands.", err),
                }
            }
        } else {
            println!(
                "Error: incorrect number of rover commands. Please check the commands entered."
            );
        }
    }
}
