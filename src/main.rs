use std::io;

struct InputCommand {
    ur_plateau: (i32, i32), // Upper right plateau coordinates
    // The String below is a list of commands for the rover
    rovers_to_deploy: Vec<(PositionAndHeading, String)>,
}

#[derive(Debug, PartialEq)]
struct PositionAndHeading {
    x: i32,
    y: i32,
    heading: char,
}

#[derive(PartialEq, Debug)]
enum RoverError {
    OutOfBounds,
    Collision,
    InvalidHeading,
    InvalidMove,
    InvalidStartMove,
}

fn main() {
    println!("Welcome to NASA's Mars Rover Simulator\n");
    println!("Please enter your commands to begin...\n");
    let mut user_input: Vec<String> = Vec::new();

    loop {
        let mut terminal_line = String::new();

        io::stdin()
            .read_line(&mut terminal_line)
            .expect("Failed to read line");

        terminal_line = terminal_line.trim().to_string();
        println!(
            "You entered: {}. Enter 'd' when you're done.",
            terminal_line
        );
        if terminal_line == *"d" {
            break;
        }
        user_input.push(terminal_line);
    }

    println!("Processing commands...\n");

    if !user_input.is_empty() {
        user_input.reverse();

        let ur_plateau = user_input.pop().unwrap();
        let ur_plateau = ur_plateau.split_whitespace().collect::<Vec<_>>();
        let ur_plateau = (
            ur_plateau[0]
                .parse::<i32>()
                .expect("Plateau coordinates need to be integers"),
            ur_plateau[1]
                .parse::<i32>()
                .expect("Plateau coordinates need to be integers"),
        );
        println!("ur_plateau: {:?}", ur_plateau);

        let input_command = InputCommand {
            ur_plateau,
            rovers_to_deploy: Vec::new(),
        };

        if !user_input.is_empty() && user_input.len() % 2 == 0 {
            println!("Processing rover commands...");
            for command in user_input {
                // let rover_instructions;
                println!("command: {}", command);
            }
        } else {
            println!("Error: incorrect number of rover commands");
        }
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
        // Help the user out by forcing the heading to uppercase
        command.0.heading = command
            .0
            .heading
            .to_uppercase()
            .to_string()
            .chars()
            .next()
            .unwrap();
        // Help the user out by forcing the commands to uppercase
        command.1 = command.1.to_uppercase();

        if command.0.x > commands.ur_plateau.0 || command.0.y > commands.ur_plateau.1 {
            return Err(RoverError::InvalidStartMove);
        }
        println!("Heading: {}, moves: {}", command.0.heading, &command.1);
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
    current_rover_positions
        .iter()
        .find(|rover| rover.x == new_rover_position.x && rover.y == new_rover_position.y)
        .is_none()
}

fn move_rover(input_command: InputCommand) -> Result<Vec<PositionAndHeading>, RoverError> {
    // TODO: test and handle invalid commands
    // TODO: test and handle start out of bounds

    let mut output = Vec::new();
    let lr_plateau = (0, 0); // Lower right plateau coordinates
    let input_command = parse_input_commands(input_command)?;

    for rovers_to_deploy in input_command.rovers_to_deploy {
        let mut current_position_and_heading = rovers_to_deploy.0;
        let commands = rovers_to_deploy.1;

        for command in commands.chars() {
            let next_heading = get_next_heading((current_position_and_heading.heading, command));
            if let Some(next_heading) = next_heading {
                current_position_and_heading.heading = next_heading;
                if command == 'M' {
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

        if will_not_collide(&current_position_and_heading, &output) {
            output.push(current_position_and_heading);
        } else {
            return Err(RoverError::Collision);
        }
    }

    Ok(output)
}

#[test]
fn test_given_spec() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![
            (
                PositionAndHeading {
                    x: 1,
                    y: 2,
                    heading: 'N',
                },
                "LMLMLMLMM".to_string(),
            ),
            (
                PositionAndHeading {
                    x: 3,
                    y: 3,
                    heading: 'E',
                },
                "MMRMMRMRRM".to_string(),
            ),
        ],
    };

    let expected_output = vec![
        PositionAndHeading {
            x: 1,
            y: 3,
            heading: 'N',
        },
        PositionAndHeading {
            x: 5,
            y: 1,
            heading: 'E',
        },
    ];
    assert_eq!(move_rover(test_input), Ok(expected_output));
}

#[test]
fn test_go_out_of_plateau_bounds_north() {
    let test_input_far_out = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'N',
            },
            "MMMMMMM".to_string(),
        )],
    };

    let test_input_on_the_border = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'N',
            },
            "MMM".to_string(),
        )],
    };

    let expected_output = Err(RoverError::OutOfBounds);

    assert_eq!(move_rover(test_input_far_out), expected_output);
    assert!(move_rover(test_input_on_the_border).is_ok());
}

#[test]
fn test_go_out_of_plateau_bounds_east() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'E',
            },
            "MMMMMMM".to_string(),
        )],
    };

    let expected_output = Err(RoverError::OutOfBounds);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_go_out_of_plateau_bounds_south() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'S',
            },
            "MMMMMMM".to_string(),
        )],
    };

    let expected_output = Err(RoverError::OutOfBounds);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_go_out_of_plateau_bounds_west() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'W',
            },
            "MMMMMMM".to_string(),
        )],
    };

    let expected_output = Err(RoverError::OutOfBounds);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_go_out_of_plateau_bounds_all() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 1,
                heading: 'N',
            },
            "MMMMMMMLMMMMMLMMMMMLMMMMMLMMMMM".to_string(),
        )],
    };

    let expected_output = Err(RoverError::OutOfBounds);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_collision_same_commands() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![
            (
                PositionAndHeading {
                    x: 1,
                    y: 2,
                    heading: 'N',
                },
                "LMLMLMLMM".to_string(),
            ),
            (
                PositionAndHeading {
                    x: 1,
                    y: 2,
                    heading: 'N',
                },
                "LMLMLMLMM".to_string(),
            ),
        ],
    };

    let expected_output = Err(RoverError::Collision);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_collision_from_different_positions() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![
            (
                PositionAndHeading {
                    x: 0,
                    y: 0,
                    heading: 'E',
                },
                "MMMMLMM".to_string(),
            ),
            (
                PositionAndHeading {
                    x: 4,
                    y: 0,
                    heading: 'N',
                },
                "MM".to_string(),
            ),
        ],
    };

    let expected_output = Err(RoverError::Collision);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_bad_command_header() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'X',
            },
            "LMLMLMLMM".to_string(),
        )],
    };

    let expected_output = Err(RoverError::InvalidHeading);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_bad_command_lowercase_header() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'n',
            },
            "LMLMLMLMM".to_string(),
        )],
    };

    let expected_output = vec![PositionAndHeading {
        x: 1,
        y: 3,
        heading: 'N',
    }];
    assert_eq!(move_rover(test_input), Ok(expected_output));
}

#[test]
fn test_bad_command_move() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'N',
            },
            "LABC".to_string(),
        )],
    };

    let expected_output = Err(RoverError::InvalidMove);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_bad_command_lowercase_move() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'N',
            },
            "LMlMLmLMM".to_string(),
        )],
    };

    let expected_output = vec![PositionAndHeading {
        x: 1,
        y: 3,
        heading: 'N',
    }];
    assert_eq!(move_rover(test_input), Ok(expected_output));
}

#[test]
fn test_bad_command_start_move_y_too_large() {
    let test_input = InputCommand {
        ur_plateau: (2, 2),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 3,
                heading: 'N',
            },
            "LRM".to_string(),
        )],
    };

    let expected_output = Err(RoverError::InvalidStartMove);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_bad_command_start_move_x_too_large() {
    let test_input = InputCommand {
        ur_plateau: (2, 2),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 3,
                y: 1,
                heading: 'N',
            },
            "LRM".to_string(),
        )],
    };

    let expected_output = Err(RoverError::InvalidStartMove);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_bad_command_start_move_x_and_y_too_large() {
    let test_input = InputCommand {
        ur_plateau: (2, 2),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 3,
                y: 3,
                heading: 'N',
            },
            "LRM".to_string(),
        )],
    };

    let expected_output = Err(RoverError::InvalidStartMove);
    assert_eq!(move_rover(test_input), expected_output);
}
