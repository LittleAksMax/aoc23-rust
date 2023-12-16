use std::fs;

const TESTMODE: bool = true;
const DAY: u8 = 1;

fn main() {
    assert!(DAY >= 1 && DAY <= 25);
    let contents: String;
    if TESTMODE {
        contents =
            fs::read_to_string(format!("day{}.txt", DAY)).expect("Should have been able to read the data file.");
    } else {
        contents =
            fs::read_to_string("test.txt").expect("Should have been able to read the test file.");
    }

    /* CODE */
}
