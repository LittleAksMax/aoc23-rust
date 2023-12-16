use std::fs;

const TESTMODE: bool = false;
const DAY: u8 = 1;

fn main() {
    assert!(DAY >= 1 && DAY <= 25);
    let contents: String;
    if !TESTMODE {
        contents =
            fs::read_to_string(format!("day{}.txt", DAY)).expect("Should have been able to read the data file.");
    } else {
        contents =
            fs::read_to_string("test2.txt").expect("Should have been able to read the test file.");
    }

    /* CODE */
    let mut total = 0;
    for line in contents.lines() {
        let mut f_init = false;
        let mut f: u32 = 0;
        let mut l: u32 = 0;
        for (i, c) in line.chars().enumerate() {
            let mut threechars: Option<&str> = None;
            let mut fourchars: Option<&str> = None;
            let mut fivechars: Option<&str> = None;

            // if the substrings exist, then create them
            if line.len() - i >= 3 {
                threechars = Some(&line[i..i + 3]);
            }
            if line.len() - i >= 4 {
                fourchars = Some(&line[i..i + 4]);
            }
            if line.len() - i >= 5 {
                fivechars = Some(&line[i..i + 5]);
            }
            let d = c.to_digit(10);
            
            let mut foundmatch = true;

            // match whatever possible valid digit or substring
            l = match d {
                Some(x) => x,
                None => 
                    match fivechars {
                        Some("three") => 3,
                        Some("seven") => 7,
                        Some("eight") => 8,
                        _ => 
                            match fourchars {
                                Some("four") => 4,
                                Some("five") => 5,
                                Some("nine") => 9,
                                _ => 
                                    match threechars {
                                        Some("one") => 1,
                                        Some("two") => 2,
                                        Some("six") => 6,
                                        _ => {
                                            foundmatch = false;
                                            l
                                        }
                                    }
                            }
                    }
            };

            if f_init == false && foundmatch == true {
                f = l;
                f_init = true;
            }
        }

        // println!("f: {}; l: {}", f, l);
        total += f * 10 + l;
    }
    println!("Total: {}", total);
}
