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
}

fn main() {
    println!("Hello, world!");
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

fn move_rover(input_command: InputCommand) -> Result<Vec<PositionAndHeading>, RoverError> {
    // TODO: test and handle rovers end up in the same position
    // TODO: test and handle invalid commands
    let mut output = Vec::new();
    let lr_plateau = (0, 0); // Lower right plateau coordinates

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

        output.push(current_position_and_heading);
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
