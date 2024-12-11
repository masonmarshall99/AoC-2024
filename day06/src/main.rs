use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("Valid file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Valid file contents");

    let grid: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect())
        .collect();
    
    println!("Distinct Positions: {}", count_path(grid.clone()));
    println!("Possible Loops: {}", count_possible_loops(grid.clone()));
}

fn count_path(mut grid: Vec<Vec<char>>) -> usize {
    let mut direction: usize = 0;
    let mut position: [usize; 2] = [0, 0];
    
    'match_direction: for (x, line) in (0..).zip(grid.iter()) { 
        for (y, ch) in (0..).zip(line.iter()) {
            match ch {
                '^' => {
                    direction = 0;
                    position = [x, y];
                    break 'match_direction;
                },
                '>' => {
                    direction = 1;
                    position = [x, y];
                    break 'match_direction;
                },
                'v' => {
                    direction = 2;
                    position = [x, y];
                    break 'match_direction;
                },
                '<' => {
                    direction = 3;
                    position = [x, y];
                    break 'match_direction;
                },
                _ => {continue;},
            }
        }
    }

    loop {
        grid[position[0]][position[1]] = 'X';
        match direction {
            0 => {
                if position[0] == 0 { break; }
                else if grid[position[0]-1][position[1]] == '#' {
                    direction = (direction + 1) % 4;
                }
                else {
                    position[0] -= 1;
                }
            },
            1 => {
                if position[1]+1 == grid[0].len() { break; }
                else if grid[position[0]][position[1]+1] == '#' {
                    direction = (direction + 1) % 4;
                }
                else {
                    position[1] += 1;
                }
            },
            2 => {
                if position[0]+1 == grid.len() { break; }
                else if grid[position[0]+1][position[1]] == '#' {
                    direction = (direction + 1) % 4;
                }
                else {
                    position[0] += 1;
                }
            },
            3 => {
                if position[1] == 0 { break; }
                else if grid[position[0]][position[1]-1] == '#' {
                    direction = (direction + 1) % 4;
                }
                else {
                    position[1] -= 1;
                }
            },
            _ => panic!("Invalid Direction"),
        }
    }

    grid.iter()
        .map(|row| 
            row.iter().filter(|ch| **ch == 'X')
            .count())
        .sum()
}

fn count_possible_loops(grid: Vec<Vec<char>>) -> usize {
    let mut direction: usize = 0;
    let mut position: [usize; 2] = [0, 0];
    let mut loops: usize = 0;
    
    'match_direction: for (x, line) in (0..).zip(grid.iter()) { 
        for (y, ch) in (0..).zip(line.iter()) {
            match ch {
                '^' => {
                    direction = 0;
                    position = [x, y];
                    break 'match_direction;
                },
                '>' => {
                    direction = 1;
                    position = [x, y];
                    break 'match_direction;
                },
                'v' => {
                    direction = 2;
                    position = [x, y];
                    break 'match_direction;
                },
                '<' => {
                    direction = 3;
                    position = [x, y];
                    break 'match_direction;
                },
                _ => {continue;},
            }
        }
    }

    for (x,line) in (0..).zip(grid.iter()) {
        for (y,_ch) in (0..).zip(line.iter()) {
            let mut temp_dir = direction.clone();
            let mut temp_grid = grid.clone();
            let mut temp_pos = position.clone();

            if temp_grid[x][y] == '.' {
                temp_grid[x][y] = '#';
            } else { continue; }
            temp_grid[position[0]][position[1]] = '.';

            loop {
                match temp_dir {
                    0 => {
                        if temp_pos[0] == 0 { break; }
                        else if temp_grid[temp_pos[0]-1][temp_pos[1]] == '#' {
                            temp_dir = (temp_dir + 1) % 4;
                        }
                        else if temp_grid[temp_pos[0]][temp_pos[1]] == '^' {
                            loops += 1;
                            break;
                        }
                        else {
                            temp_grid[temp_pos[0]][temp_pos[1]] = '^';
                            temp_pos[0] -= 1;
                        }
                    },
                    1 => {
                        if temp_pos[1]+1 == temp_grid[0].len() { break; }
                        else if temp_grid[temp_pos[0]][temp_pos[1]+1] == '#' {
                            temp_dir = (temp_dir + 1) % 4;
                        }
                        else if temp_grid[temp_pos[0]][temp_pos[1]] == '>' {
                            loops += 1;
                            break;
                        }
                        else {
                            temp_grid[temp_pos[0]][temp_pos[1]] = '>';
                            temp_pos[1] += 1;
                        }
                    },
                    2 => {
                        if temp_pos[0]+1 == temp_grid.len() { break; }
                        else if temp_grid[temp_pos[0]+1][temp_pos[1]] == '#' {
                            temp_dir = (temp_dir + 1) % 4;
                        }
                        else if temp_grid[temp_pos[0]][temp_pos[1]] == 'v' {
                            loops += 1;
                            break;
                        }
                        else {
                            temp_grid[temp_pos[0]][temp_pos[1]] = 'v';
                            temp_pos[0] += 1;
                        }
                    },
                    3 => {
                        if temp_pos[1] == 0 { break; }
                        else if temp_grid[temp_pos[0]][temp_pos[1]-1] == '#' {
                            temp_dir = (temp_dir + 1) % 4;
                        }
                        else if temp_grid[temp_pos[0]][temp_pos[1]] == '<' {
                            loops += 1;
                            break;
                        }
                        else {
                            temp_grid[temp_pos[0]][temp_pos[1]] = '<';
                            temp_pos[1] -= 1;
                        }
                    },
                    _ => panic!("Invalid Direction"),
                }
            }
            
            //Debug by printing finished grid
            //println!("Placed at: {x},{y}");
            //for line in temp_grid.iter() {
            //    let mut char_line = String::new();
            //    for ch in line.iter() {
            //        char_line.push(*ch);
            //    }
            //    println!("{}", char_line);
            //}

        }
    }
    loops
}
