use rusty_rover::rover::{move_rover, parse_user_plateau, InputCommand, PositionAndHeading, RoverError};

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
fn test_bad_command_move_handle_extra_spaces() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'N',
            },
            "L M LML MLM  M".to_string(),
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
fn test_bad_command_move_handle_extra_non_alphanumerics() {
    let test_input = InputCommand {
        ur_plateau: (5, 5),
        rovers_to_deploy: vec![(
            PositionAndHeading {
                x: 1,
                y: 2,
                heading: 'N',
            },
            "LMLM%LM4LM'M".to_string(),
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

    let expected_output = Err(RoverError::StartOutOfBounds);
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

    let expected_output = Err(RoverError::StartOutOfBounds);
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

    let expected_output = Err(RoverError::StartOutOfBounds);
    assert_eq!(move_rover(test_input), expected_output);
}

#[test]
fn test_parse_user_plateau_standard() {
    let test_input = "5 5".to_string();
    let expected_output = Ok((5, 5));
    assert_eq!(parse_user_plateau(test_input), expected_output);
}

#[test]
fn test_parse_user_plateau_no_spaces() {
    let test_input = "55".to_string();
    let expected_output = Ok((5, 5));
    assert_eq!(parse_user_plateau(test_input), expected_output);
}

#[test]
fn test_parse_user_plateau_empty() {
    let test_input = "".to_string();
    let expected_output = Err(RoverError::InvalidPlateau);
    assert_eq!(parse_user_plateau(test_input), expected_output);
}

#[test]
fn test_parse_user_plateau_missing_coordinate() {
    let test_input = "4".to_string();
    let expected_output = Err(RoverError::InvalidPlateau);
    assert_eq!(parse_user_plateau(test_input), expected_output);
}

#[test]
fn test_parse_user_plateau_special_chars() {
    let test_input = "#5$5..;".to_string();
    let expected_output = Ok((5, 5));
    assert_eq!(parse_user_plateau(test_input), expected_output);
}
