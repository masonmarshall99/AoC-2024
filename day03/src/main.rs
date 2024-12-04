use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()>{
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let sum = mul_scan(&contents); 
    println!("Mul Total: {sum}");

    let sum = do_dont_scan(&contents);
    println!("Do/Don't Total: {sum}");
    Ok(())
}

fn mul_scan(haystack: &String) -> i32 {

    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let pairs: Vec<(i32, i32)> = pattern.captures_iter(haystack).map(|caps| {
        let (_, [a, b]) = caps.extract();
        let num_a: i32 = a.parse().expect("Valid number, digits are captured");
        let num_b: i32 = b.parse().expect("Valid number, digits are captured");
        (num_a, num_b)
    }).collect();
    
    let mut sum = 0;
    for (a, b) in pairs.iter() {
        sum += a * b;
    }
    sum
}

fn do_dont_scan(haystack: &String) -> i32 {
    let command_pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let command_captures = command_pattern.captures_iter(haystack);

    let mut sum = 0;
    let mut mul = 1;

    for command in command_captures {
        let (comm, []) = command.extract();
        if comm == "do()" {
            mul = 1;
        } else if comm == "don't()" {
            mul = 0;
        } else {
            let (_, [a, b]) = mul_pattern.captures(comm).expect("Valid match").extract();
            let num_a: i32 = a.parse().expect("Valid number");
            let num_b: i32 = b.parse().expect("Valid number");
            sum += num_a * num_b * mul;
        } 
    }
    sum
}
