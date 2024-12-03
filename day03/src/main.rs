use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()>{
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let pairs: Vec<(i32, i32)> = pattern.captures_iter(&contents).map(|caps| {
        let (_, [a, b]) = caps.extract();
        let num_a: i32 = a.parse().expect("Valid number, digits are captured");
        let num_b: i32 = b.parse().expect("Valid number, digits are captured");
        (num_a, num_b)
    }).collect();
    
    let mut sum = 0;
    for (a, b) in pairs.iter() {
        sum += a * b;
    }
    
    println!("Total: {sum}");
    
    Ok(())
}

