struct Input {
    ur_plateau: (i32, i32),
    rovers_deployed: Vec<(PositionAndHeading, String)>,
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

fn move_rover(command: Input) -> Vec<PositionAndHeading> {
    vec![PositionAndHeading {
        x: 0,
        y: 0,
        heading: 'N',
    }]
}

#[test]
fn test_given_spec() {
    let test_input = Input {
        ur_plateau: (5, 5),
        rovers_deployed: vec![
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
