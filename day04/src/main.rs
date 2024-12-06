use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let xmas_count = xmas_search(&contents);
    println!("XMAS Count: {xmas_count}");
    
    let x_mas_count = x_mas_search(&contents);
    println!("X-MAS Count: {x_mas_count}");

    Ok(())
}

fn xmas_search(contents: &str) -> i32 {
    let mut sum = 0;
    let grid: Vec<Vec<char>> = contents.lines()
        .map(|l| l.chars().collect())
        .collect();

    for (x, line) in grid.iter().enumerate() {
        for (y, _cha) in line.iter().enumerate() {
            sum += xmas_search_rec(&grid, x.try_into().unwrap(), y.try_into().unwrap(), [0,0], 0);
        }
    }

    sum
}

fn xmas_search_rec(grid: &Vec<Vec<char>>,  x: i32, y: i32, direction: [i32;2], index: usize) -> i32 {
    //println!("x: {}, y: {}, d0: {}, d1: {}", x, y, direction[0], direction[1]);
    let xmas = ['X', 'M', 'A', 'S'];
    let mut val = 0;
    if index >= 4 { 
        return 1;
    } else if index == 0 && grid[TryInto::<usize>::try_into(x).unwrap()][TryInto::<usize>::try_into(y).unwrap()] == xmas[0] {
        for a in -1..=1 {
            for b in -1..=1 {
                val += xmas_search_rec(grid, x, y, [a, b], index+1); 
            }
        }
    } else if index > 0 
        && x + direction[0] >= 0 && x + direction[0] < grid.len().try_into().unwrap()
        && y + direction[1] >= 0 && y + direction[1] < grid[0].len().try_into().unwrap()
        && grid[TryInto::<usize>::try_into(x + direction[0]).unwrap()][TryInto::<usize>::try_into(y + direction[1]).unwrap()] == xmas[index]
    {
        val += xmas_search_rec(grid, x + direction[0], y + direction[1], direction, index+1);
    }
    val
}

fn x_mas_search(contents: &str) -> i32{
    let mut sum = 0;
    let grid: Vec<Vec<char>> = contents.lines()
        .map(|l| l.chars().collect())
        .collect();
    for (x, line) in grid.iter().enumerate() {
        for (y, ch) in line.iter().enumerate() {
            // Valid position for 'A'
            if *ch == 'A' && 
            x > 0 && x + 1 < grid.len() &&
            y > 0 && y + 1 < grid[0].len() {
                let ul = grid[x-1][y-1];
                let br = grid[x+1][y+1];
                let bl = grid[x+1][y-1];
                let ur = grid[x-1][y+1];

                if ((ul == 'M' && br == 'S') || (ul == 'S' && br == 'M')) &&
                ((bl == 'M' && ur == 'S') || (bl == 'S' && ur == 'M')) {
                    sum += 1;
                }
            }
        }
    }
    sum
}
