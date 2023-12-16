use std::fs;

const TESTMODE: bool = false;
const DAY: u8 = 2;

// 12 red cubes, 13 green cubes, and 14 blue cubes?
const RED  : u8 = 12;
const GREEN: u8 = 13;
const BLUE : u8 = 14;

fn main() {
    assert!(DAY >= 1 && DAY <= 25);
    let contents: String;
    if !TESTMODE {
        contents =
            fs::read_to_string(format!("day{}.txt", DAY)).expect("Should have been able to read the data file.");
    } else {
        contents =
            fs::read_to_string("test.txt").expect("Should have been able to read the test file.");
    }

    /* CODE */
    let mut total = 0;

    for (game_idx, line) in contents.lines().enumerate() {
        // separate game_data from input line (i.e., stuff after 'Game xyz: ')
        let game_id = game_idx + 1;
        let game_data = &line[format!("Game {}: ", game_id).len()..];
        // println!("{}", game_data);

        let mut impossible = false;
        // iterate through moves in game
        for handful in game_data.split("; ") {
            // just skip each consequent handful if we already
            // decided the game is impossible
            if impossible {
                continue;
            }

            let mut red: u8 = 0;
            let mut green: u8 = 0;
            let mut blue: u8 = 0;

            for group in handful.split(", ") {
                let num_and_clr: Vec<&str> = group.split(' ').collect();

                assert!(num_and_clr.len() == 2, "`num_and_clr` must have 2 elements exactly.");

                let num = match (*num_and_clr.first().unwrap()).parse::<u8>() {
                    Ok(x) => x,
                    _ => { // this should never happen
                        assert!(false, "Must be able to parse into from first in each group.");
                        0 // impossible output just to please compiler
                    }
                };

                assert!(num > 0, "`num` can't be 0.");

                let clr = *num_and_clr.last().unwrap();

                match clr {
                    "red" => {
                        red += num;
                    },
                    "green" => {
                        green += num;
                    },
                    "blue" => {
                        blue += num;
                    },
                    _ => {
                        assert!(false, "`clr` must be one of 'red', 'green', 'blue'.");
                    }
                }
            }

            // this is an impossible configuration
            if red > RED || green > GREEN || blue > BLUE {
                // println!("Game ID: {} (R, G, B): ({}, {}, {})", game_id, red, green, blue);
                impossible = true;
            }
        }

        if !impossible {
            total += game_id;
        }
    }
    println!("Sum of IDs: {}", total);
}
