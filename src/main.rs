use std::{fs, collections::HashSet};

const TESTMODE: bool = false;
const DAY: u8 = 4;

// the 'Game xyz: ' has a different length in the different modes
macro_rules! substr_start {
    () => { if TESTMODE { 8 } else { 10 } };
}

macro_rules! games {
    () => { if TESTMODE { 6 } else { 219 } };
}

macro_rules! expand_cards {
    ($slice:expr) => {
        (*$slice.expect("Slice must exist."))
            // split over spaces, filter out bad splits (double space) and map each remaining
            // value to their parsed integer forms after trimming (just in case)
            .split_ascii_whitespace().map(|x| match x.trim().parse::<u32>() {
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
    // cards stores [Game no. - 1] => instances (default = 1)
    let mut cards = [1_u32; games!()];
    for (game_idx, line) in contents.lines().enumerate() {
        let data: Vec<&str> = (&line[substr_start!()..]).split(" | ").collect();
        let winners: HashSet<u32> = expand_cards!(data.first());
        let owned: HashSet<u32> = expand_cards!(data.last());

        let matches = owned.intersection(&winners).count() as u32;

        for i in (game_idx + 1)..(game_idx + matches as usize + 1) {
            // each subsequent gets as many copies as we have copies of this card
            cards[i] += cards[game_idx];
        }
    }

    // println!("Total: {}", total);
    println!("Number of cards: {}", cards.iter().sum::<u32>());
}
