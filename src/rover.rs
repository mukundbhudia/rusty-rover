#[derive(Debug)]
pub struct InputCommand {
    pub ur_plateau: (i32, i32), // Upper right plateau coordinates
    // The String below is a list of commands/moves for the rover
    pub rovers_to_deploy: Vec<(PositionAndHeading, String)>,
}

#[derive(Debug, PartialEq)]
pub struct PositionAndHeading {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

#[derive(PartialEq, Debug)]
pub enum RoverError {
    OutOfBounds,
    Collision,
    InvalidHeading,
    InvalidMove,
    InvalidStartMove,
    InvalidStartPosition,
    InvalidPlateau,
    InvalidNumberOfCommandsForRover,
    StartOutOfBounds,
}

pub fn print_final_rover_positions(positions: Vec<PositionAndHeading>) {
    println!("\nFinal rover position(s):\n");
    for position in positions {
        println!("{} {} {}", position.x, position.y, position.heading);
    }
}

pub fn parse_user_plateau(plateau: String) -> Result<(i32, i32), RoverError> {
    // Help the user by stripping out non digit chars
    let plateau = plateau
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as i32)
        .collect::<Vec<_>>();

    if plateau.len() < 2 {
        // Expecting only a 2d coordinate
        Err(RoverError::InvalidPlateau)
    } else {
        Ok((plateau[0], plateau[1]))
    }
}

pub fn parse_rover_to_deploy(
    ur_plateau: (i32, i32),
    rovers: Vec<String>,
) -> Result<InputCommand, RoverError> {
    let mut input_command = InputCommand {
        ur_plateau,
        rovers_to_deploy: Vec::new(),
    };

    // Process each rover command as a paired set of start position and moves
    if !rovers.is_empty() && rovers.len() % 2 == 0 {
        for command in rovers.chunks(2) {
            // Help the user out by accepting only alphanumeric chars
            let rover_start_position = command[1]
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<Vec<_>>();

            if let Ok(rover_start) = parse_rover_commands(rover_start_position) {
                let rover_start_position = (rover_start, command[0].to_string());
                input_command.rovers_to_deploy.push(rover_start_position);
            } else {
                return Err(RoverError::InvalidStartPosition);
            }
        }
        // Reverse back from 'stack' to vec. TODO: a better way than this?
        input_command.rovers_to_deploy.reverse();
    } else {
        return Err(RoverError::InvalidNumberOfCommandsForRover);
    }
    Ok(input_command)
}

fn parse_rover_commands(commands: Vec<char>) -> Result<PositionAndHeading, RoverError> {
    // Rover position must be 3 chars: x, y coordinate and a heading
    if commands.len() == 3 {
        let output = PositionAndHeading {
            x: match commands[0].to_digit(10) {
                Some(x) => x as i32,
                None => return Err(RoverError::InvalidStartMove),
            },
            y: match commands[1].to_digit(10) {
                Some(x) => x as i32,
                None => return Err(RoverError::InvalidStartMove),
            },
            heading: commands[2],
        };
        Ok(output)
    } else {
        Err(RoverError::InvalidMove)
    }
}

fn is_valid_heading(heading: char) -> bool {
    matches!(heading, 'N' | 'E' | 'S' | 'W')
}

fn is_valid_move(move_to_check: &str) -> bool {
    move_to_check
        .chars()
        .find(|x| !matches!(x, 'L' | 'R' | 'M'))
        .is_none()
}

fn parse_input_commands(commands: InputCommand) -> Result<InputCommand, RoverError> {
    let mut fixed_input_command = InputCommand {
        ur_plateau: commands.ur_plateau,
        rovers_to_deploy: Vec::new(),
    };

    for command in commands.rovers_to_deploy {
        let mut command = command;
        // Help the user by forcing the heading to uppercase
        command.0.heading = command
            .0
            .heading
            .to_uppercase()
            .to_string()
            .chars()
            .next()
            .unwrap();
        // Help the user by forcing the commands to uppercase and removing non-alphabetic chars
        command.1 = command
            .1
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

        if command.0.x > commands.ur_plateau.0 || command.0.y > commands.ur_plateau.1 {
            return Err(RoverError::StartOutOfBounds);
        }
        if !is_valid_heading(command.0.heading) {
            return Err(RoverError::InvalidHeading);
        }
        if !is_valid_move(&command.1) {
            return Err(RoverError::InvalidMove);
        }
        fixed_input_command.rovers_to_deploy.push(command);
    }
    Ok(fixed_input_command)
}

fn get_next_heading(heading_and_rotation: (char, char)) -> Option<char> {
    // Given current heading and intended next move, map the next heading
    match heading_and_rotation {
        ('N', 'L') => Some('W'),
        ('N', 'R') => Some('E'),
        ('E', 'L') => Some('N'),
        ('E', 'R') => Some('S'),
        ('S', 'L') => Some('E'),
        ('S', 'R') => Some('W'),
        ('W', 'L') => Some('S'),
        ('W', 'R') => Some('N'),
        (heading, 'M') => Some(heading), // Moves don't mutate headings
        (_, _) => None,
    }
}

fn will_not_collide(
    new_rover_position: &PositionAndHeading,
    current_rover_positions: &[PositionAndHeading],
) -> bool {
    // Check that for all rover positions, is the next rover in the same position
    current_rover_positions
        .iter()
        .find(|current_rover_position| {
            current_rover_position.x == new_rover_position.x
                && current_rover_position.y == new_rover_position.y
        })
        .is_none()
}

pub fn simulate_rover_move(
    input_command: InputCommand,
) -> Result<Vec<PositionAndHeading>, RoverError> {
    let mut output = Vec::new();
    let lr_plateau = (0, 0); // Lower right plateau coordinates
    let input_command = parse_input_commands(input_command)?;

    for rovers_to_deploy in input_command.rovers_to_deploy {
        // Keep the current rover state in memory to mutate as moves are processed
        let mut current_position_and_heading = rovers_to_deploy.0;

        for command in rovers_to_deploy.1.chars() {
            let next_heading = get_next_heading((current_position_and_heading.heading, command));

            if let Some(next_heading) = next_heading {
                current_position_and_heading.heading = next_heading;
                // Apply moves if move command found
                if command == 'M' {
                    // Move relevant coordinate depending on heading and checking
                    // if the next move is within the plateau bounds
                    match next_heading {
                        'N' => {
                            if current_position_and_heading.y < input_command.ur_plateau.1 {
                                current_position_and_heading.y += 1
                            } else {
                                return Err(RoverError::OutOfBounds);
                            }
                        }
                        'E' => {
                            if current_position_and_heading.x < input_command.ur_plateau.0 {
                                current_position_and_heading.x += 1
                            } else {
                                return Err(RoverError::OutOfBounds);
                            }
                        }
                        'S' => {
                            if current_position_and_heading.y > lr_plateau.1 {
                                current_position_and_heading.y -= 1
                            } else {
                                return Err(RoverError::OutOfBounds);
                            }
                        }
                        'W' => {
                            if current_position_and_heading.x > lr_plateau.0 {
                                current_position_and_heading.x -= 1
                            } else {
                                return Err(RoverError::OutOfBounds);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        // With all the moves applied, check if the rover won't collide to existing rovers
        if will_not_collide(&current_position_and_heading, &output) {
            output.push(current_position_and_heading);
        } else {
            return Err(RoverError::Collision);
        }
    }

    Ok(output)
}
