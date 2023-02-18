use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    // Parse input params
    let path = env::args().nth(1).expect("No path provided.");
    
    // Load calories file
    println!("Loading calories from: {}", path);
    let data = io::BufReader::new(
        fs::File::open(path)
            .expect("Could not read file!")
        ).lines();
        

    let mut elves: Vec<u32> = Vec::new();

    let mut accumulator: u32 = 0;

    for line in data {
        if let Ok(snack) = line {
            match snack.as_str() {
                "" => {
                    elves.push(accumulator);
                    accumulator = 0;
                },
                other => accumulator += match other.parse::<u32>() {
                    Ok(calories) => calories,
                    _ => 0
                }
            }
        }
    }

    elves.sort();
    elves.reverse();

    println!("{:#?}", &elves[..3]);
    println!(
        "Sum: {}", 
        elves.iter().take(3).sum::<u32>()
    )

}
