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

fn move_rover(command: InputCommand) -> Vec<PositionAndHeading> {
    // TODO: test and handle going out of plateau bounds
    // TODO: test and handle rovers end up in the same position
    // TODO: test and handle invalid commands
    let mut output = Vec::new();
    for rovers_to_deploy in command.rovers_to_deploy {
        let mut current_position_and_heading = rovers_to_deploy.0;
        let commands = rovers_to_deploy.1;

        for command in commands.chars() {
            let next_heading = get_next_heading((current_position_and_heading.heading, command));
            if let Some(heading) = next_heading {
                current_position_and_heading.heading = heading;
                if command == 'M' {
                    match heading {
                        'N' => current_position_and_heading.y += 1,
                        'E' => current_position_and_heading.x += 1,
                        'S' => current_position_and_heading.y -= 1,
                        'W' => current_position_and_heading.x -= 1,
                        _ => {}
                    }
                }
            }
        }

        output.push(current_position_and_heading);
    }
    output
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
    assert_eq!(move_rover(test_input), expected_output);
}
