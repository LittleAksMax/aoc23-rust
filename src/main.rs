use std::fs;

const TESTMODE: bool = false;
const DAY: u8 = 2;

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
    let mut total: usize = 0;

    for (game_idx, line) in contents.lines().enumerate() {
        // separate game_data from input line (i.e., stuff after 'Game xyz: ')
        let game_id = game_idx + 1;
        let game_data = &line[format!("Game {}: ", game_id).len()..];
        // println!("{}", game_data);

        // store the fewest required of each colour
        let mut min_red  : usize = 0;
        let mut min_green: usize = 0;
        let mut min_blue : usize = 0;
        
        // iterate through moves in game
        for handful in game_data.split("; ") {
            for group in handful.split(", ") {
                let num_and_clr: Vec<&str> = group.split(' ').collect();

                assert!(num_and_clr.len() == 2, "`num_and_clr` must have 2 elements exactly.");

                let num = match (*num_and_clr.first().unwrap()).parse::<usize>() {
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
                        if num > min_red {
                            min_red = num;
                        }
                    },
                    "green" => {
                        if num > min_green {
                            min_green = num;
                        }
                    },
                    "blue" => {
                        if num > min_blue {
                            min_blue = num;
                        }
                    },
                    _ => {
                        assert!(false, "`clr` must be one of 'red', 'green', 'blue'.");
                    }
                }
            }
        }

        // add power to total
        total += min_red * min_green * min_blue;
    }
    println!("Sum of IDs: {}", total);
}
