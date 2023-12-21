use std::fs;

const TESTMODE: bool = false;
const DAY: u8 = 5;

struct LineData {
    src: u64,
    dest: u64,
    rng: u64
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
    let sections: Vec<&str> = contents.split("\n\n").collect();

    // 7 is the length of 'seeds: '
    let seed_data: Vec<u64> = sections[0]
        .split(' ')
        // filter out the lines with non-digit chars by removing
        // everything where case matters
        .filter(|&x| x.to_ascii_lowercase() == x.to_ascii_uppercase())
        .map(|x| x.parse::<u64>().expect("Must parse seed correctly."))
        .collect();

    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let mut seed_iter = seed_data.iter();
    for _ in 0..(seed_data.len() / 2) {
        seeds.push((*seed_iter.next().unwrap(), *seed_iter.next().unwrap()));
    }

    let mut data: [Vec<LineData>; 7] = Default::default();
    for (idx, &section) in sections[1..].iter().enumerate() {
        let mut section_line_data: Vec<LineData> = Vec::new();
        for line in section.split('\n') {
            // skip if it is a heading line (it contains non-digit characters)
            if line.to_ascii_lowercase() != line.to_ascii_uppercase() {
                continue;
            }
            let line_data: Vec<u64> = line.split(' ')
                .map(|x| x.parse::<u64>().expect("Should parse line data correctly."))
                .collect();
            assert!(line_data.len() == 3, "Each line must have 3 data items.");
            
            // println!("Dest: {}; Src: {}; Range: {}", line_data[0], line_data[1], line_data[2]);

            section_line_data.push(LineData {src: line_data[1], dest: line_data[0], rng: line_data[2]});
        }
        data[idx] = section_line_data;
    }

    let mut finals: Vec<u64> = Vec::new();
    for (start, range) in seeds {
        for seed in start..(start + range + 1) {
            let mut current = seed;
            for section in data.iter() {
                let mut found_mapping = false;
                for mapping in section {
                    // are we in the mapping rule range
                    if !found_mapping && current >= mapping.src && current <= mapping.src + mapping.rng {
                        // print!("{} {} {} => ", mapping.dest, mapping.src, mapping.rng);
                        current = mapping.dest + current - mapping.src;
                        found_mapping = true;
                    }
                }
                // println!("{} (seed {})", current, seed);
            }
            // println!();
            println!("{} -> {}", seed, current);
            finals.push(current);
        }
    }
    println!("Min location: {}", finals.iter().min().unwrap());
}
