use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut list1: Vec<i32> = Vec::new(); 
    let mut list2: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        list1.push( nums[0].parse().expect("Should be valid number") );
        list2.push( nums[1].parse().expect("Should be valid number") );
    }
    
    let diff = difference_score(list1.clone(), list2.clone());
    println!("Difference Score: {diff}");

    let sim = similarity_score(list1.clone(), list2.clone());
    println!("Similarity Score: {sim}");

    Ok(())
}

fn difference_score(mut l1: Vec<i32>, mut l2: Vec<i32>) -> i32 {
    let mut v = Vec::new();
    l1.sort();
    l2.sort();
    
    for (a, b) in l1.iter().zip(l2.iter()) {
        v.push( (a-b).abs() );
    }
    
    let mut sum = 0;
    for val in v {
        sum += val;
    }
    sum
}

fn similarity_score(l1: Vec<i32>, l2: Vec<i32>) -> i32 {
    let mut s = 0;
    for i in l1.iter() {
        for j in l2.iter() {
            if i == j {
                s += i;
            }
        }
    }
    s
}
