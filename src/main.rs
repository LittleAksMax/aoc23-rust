use std::{fs, collections::HashSet};

const TESTMODE: bool = false;
const DAY: u8 = 3;

const INPUT_COLS: i32 = 140;
const TEST_INPUT_COLS: i32 = 140;

macro_rules! max_col {
    () => (if TESTMODE { TEST_INPUT_COLS } else { INPUT_COLS })
}

macro_rules! hash {
    ($row:expr, $col:expr) => ($row * max_col!() + $col);
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
    // loop through and add all the locations of the symbols
    let mut syms: HashSet<i32> = HashSet::new();
    for (row, line) in contents.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            // println!("{} at {}", c, hash!(row, col));
            // add all coordinates to hashset where it is a
            // symbol (i.e., not '.' and not digit)
            match c {
                '.' | '0'..='9' => { },
                _ => {
                    let sym_hash = hash!(row as i32, col as i32);
                    syms.insert(sym_hash);
                }
            }
        }
    }

    // println!("Length: {}", syms.len());

    // loop through and parse part numbers and add to running total
    let mut total: u32 = 0;
    let mut in_num = false;
    let mut num: u32 = 0; // number being constructed
    let mut is_part = false;
    for (row, line) in contents.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let dopt = c.to_digit(10);
            let d: u32;
            
            if dopt.is_some() {
                d = dopt.expect("Must be some to be expected");

                if in_num == false {
                    num = d;
                    in_num = true;
                } else {
                    num *= 10;
                    num += d;
                }
            } else { // dopt is None
                // if we were in_num, then we have fully
                // constructed the part number
                if in_num {                   
                    // print!("Constructed {} at ({}, {})", num, row, col);
                    in_num = false;
                    if is_part {
                        // print!(" and it's a part!");
                        total += num;
                        is_part = false;
                    }
                    // println!();
                }
            }

            // skip if it isn't a digit
            if in_num == false {
                continue;
            }

            // figure out if there is a surrounding symbol
            // to check if this number corresponds to a part
            for i in -1..2_i32 {
                for j in -1..2_i32 {
                    let checked_row = row as i32 + i;
                    let checked_col = col as i32 + j;
                    // println!("({checked_col}, {checked_row})");
                    let sym_hash = hash!(checked_row, checked_col);
                    if syms.contains(&sym_hash) {
                        is_part = true;
                    }
                }
            }
        }
    }

    println!("Total: {}", total);
}
