use std::{fs, collections::{HashMap, HashSet}};

const TESTMODE: bool = false;
const DAY: u8 = 3;

const INPUT_COLS: i32 = 140;
const TEST_INPUT_COLS: i32 = 10;

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
    let mut gears: HashMap<i32, Vec<u32>> = HashMap::new();

    for (row, line) in contents.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            // println!("{} at {}", c, hash!(row, col));
            // add all coordinates to hashset where it is a
            // symbol (i.e., not '.' and not digit)
            match c {
                '*' => {
                    let sym_hash = hash!(row as i32, col as i32);
                    syms.insert(sym_hash);
                    gears.insert(sym_hash, Vec::new());
                },
                _ => ()
            }
        }
    }

    // println!("Length: {}", syms.len());

    // loop through and parse part numbers and add to running total
    let mut total: u32 = 0;
    let mut in_num = false;
    let mut num: u32 = 0; // number being constructed
    let mut gear: Option<i32> = None;
    for (row, line) in contents.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let dopt = c.to_digit(10);

            if let Some(d) = dopt {
                if !in_num {
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
                    if gear.is_some() {
                        let gear_val = gear.expect("There must be a gear.");
                        // print!(" and it's a part!");
                        let gear_vec_opt = gears.get_mut(&gear_val);
                        let some_gear_vec = gear_vec_opt.expect("Corresponding Vec<u32> must exist.");
                        some_gear_vec.push(num);
                        gear = None;
                    }
                    // println!();
                }
            }

            // skip if it isn't a digit
            if !in_num {
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
                        gear = Some(sym_hash);
                    }
                }
            }
        }
    }

    for (_, v) in gears {
        if v.len() == 2 {
            let mut prod = 1;
            for val in v {
                prod *= val;
            }
            total += prod;
        }
    }
    println!("Total: {}", total);
}
