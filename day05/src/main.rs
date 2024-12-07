use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let split: Vec<&str> = contents.split("\n\n").collect();
    let rules: Vec<Vec<i32>> = split[0].lines()
        .map(|line| line.split("|")
            .map(|num| num.parse().expect("Valid Number"))
            .collect())
        .collect();
    let orders: Vec<Vec<i32>> = split[1].lines()
        .map(|line| line.split(",")
            .map(|num| num.parse().expect("Valid Number"))
            .collect())
        .collect();
    
    let total = part_one(&rules, &orders);
    println!("Correct Total: {total}");

    let total = part_two(&rules, &orders);
    println!("Incorrect Total: {total}");

    Ok(())
}


fn part_one(rules: &Vec<Vec<i32>>, orders: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for order in orders {
        let mut valid = true;
        
        for rule in rules {
            let r0_opt = order.iter().position(|x| *x == rule[0]);
            let r1_opt = order.iter().position(|x| *x == rule[1]);
            match (r0_opt, r1_opt) {
                (Some(r0), Some(r1)) => {
                    if r1 < r0 {
                        valid = false;
                    }
                }
                _ => {}
            }
        }
        
        if valid {
            sum += order[order.len()/2]; //Input orders are all odd length, so this is fine
        }

    }
    sum
}

fn part_two(rules: &Vec<Vec<i32>>, orders: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for order in orders {
        let mut valid = true;
        
        for rule in rules {
            let r0_opt = order.iter().position(|x| *x == rule[0]);
            let r1_opt = order.iter().position(|x| *x == rule[1]);
            match (r0_opt, r1_opt) {
                (Some(r0), Some(r1)) => {
                    if r1 < r0 {
                        valid = false;
                    }
                }
                _ => {}
            }
        }
        
        if !valid {
            //reorder to find the new middle number
            //add middle number to sum
            let  mut order_clone = order.clone();
            'validity: while !valid {
                for rule in rules {
                    let r0_opt = order_clone.iter().position(|x| *x == rule[0]);
                    let r1_opt = order_clone.iter().position(|x| *x == rule[1]);
                    match (r0_opt, r1_opt) {
                        (Some(r0), Some(r1)) => {
                            if r1 < r0 {
                                order_clone.swap(r0, r1);
                                continue 'validity;
                            }
                        }
                        _ => {}
                    }
                }
                valid = true;
            }
            sum += order_clone[order_clone.len()/2]; 
        }

    }
    sum
}
