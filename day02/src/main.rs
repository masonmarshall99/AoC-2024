use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let mut v: Vec<i32> = Vec::new();
        for num in line.split_whitespace() {
           v.push( num.parse().expect("Should be valid number") ); 
        }
        reports.push(v);
    }

    let safe = safe_reports(reports.clone());
    println!("Safe Reports: {safe}");
    
    let dampened = dampened_safe_reports(reports.clone());
    println!("Dampened Safe Reports: {dampened}");

    Ok(())
}

fn safe_reports(reports: Vec<Vec<i32>>) -> i32 {
    let mut s = 0;
    
    for report in reports.iter() {
        let mut valid = true;
        
        let mut iter = report.iter();
        let mut prev = iter.next().expect("At least one value per report");
        let mut asc: Vec<bool> = Vec::new(); //Relationship between report[x] and report[x+1]
        
        // Difference between 1 and 3
        for level in iter {
            let diff = (level-prev).abs();
            if  !((1..=3).contains(&diff)) {
                valid = false;
            }
            asc.push(level > prev);
            prev = level;
        }

        // Test if all in asc are not the same, i.e. not all asc or desc
        if !(asc.iter().all(|&x| x == asc[0])) {valid = false;}

        if valid { s += 1; }
    }

    s
}

fn dampened_safe_reports(reports: Vec<Vec<i32>>) -> i32 {
    // Same conditions as safe reports, except it can tolerate a single problematic level
    let mut s = 0;
    
    for report in reports.iter() {
        let mut valid = false;

        for i in 0..=report.len() {
            let mut r = report.clone();
            if i != report.len() { r.remove(i); }
            
            let mut cur_valid = true;

            let mut iter = r.iter();
            let mut prev = iter.next().expect("At least one value per report");
            let mut asc: Vec<bool> = Vec::new(); //Relationship between report[x] and report[x+1]
            
            // Difference between 1 and 3
            for level in iter {
                let diff = (level-prev).abs();
                if  !((1..=3).contains(&diff)) {
                    cur_valid = false;
                }
                asc.push(level > prev);
                prev = level;
            }

            // Test if all in asc are not the same, i.e. not all asc or desc
            if !(asc.iter().all(|&x| x == asc[0])) {cur_valid = false;}

            // Valid subset
            if cur_valid { 
                valid = true;
                break;
            }
        }

        if valid { s += 1; }
    }

    s
}
