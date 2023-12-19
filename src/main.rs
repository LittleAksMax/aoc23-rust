use std::{fs, collections::HashSet};

const TESTMODE: bool = true;
const DAY: u8 = 4;

macro_rules! expand_cards {
    ($slice:expr) => {
        (*$slice.expect("Slice must exist."))
            // split over spaces, filter out bad splits (double space) and map each remaining
            // value to their parsed integer forms after trimming (just in case)
            .split(' ').filter(|&x| !x.is_empty()).map(|x| match x.trim().parse::<u32>() {
                Ok(n) => n,
                _ => {
                    assert!(false, "Must be able to parse properly.");
                    0 // some return value to please compiler
                }
            }).collect()
    }
}

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
    for line in contents.lines() {
        let data: Vec<&str> = (&line[11..]).split(" | ").collect();
        let winners: Vec<u32> = expand_cards!(data.first());
        let owned: Vec<u32> = expand_cards!(data.last());
    }
}
