use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn p2() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<String>> = Vec::new();
    let mut total_count = 0;

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        grid.push(row);
    }

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == "@" {
                    if have_less_than_4_papers_fn(&grid, i, j) {
                        to_remove.push((i, j));
                        // grid[i][j] = "x".to_string();
                        // total_count += 1;
                    }
                }
            }
        }
        total_count += to_remove.len();
        if to_remove.len() == 0 {
            break;
        }

        for (x, y) in to_remove {
            grid[x][y] = "x".to_string();
        }
    }

    println!(
        "Total count of positions with less than 4 papers around: {}",
        total_count
    );
}

fn have_less_than_4_papers_fn(grid: &Vec<Vec<String>>, x: usize, y: usize) -> bool {
    // let direction_up: (usize, usize)=(x-1, y);
    // let direction_down=(x+1, y);
    // let direction_left=(x, y-1);
    // let direction_right=(x, y+1);

    // let directions = [direction_up, direction_down, direction_left, direction_right];
    // update directions to handle all 8 directions
    // let directions: [(isize, isize); 4]=[(0isize,-1),(0,1),(-1,0),(1,0)];
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1), // Top row
        (0, -1),
        (0, 1), // Middle row (excluding center)
        (1, -1),
        (1, 0),
        (1, 1), // Bottom row
    ];
    let mut paper_count = 0;
    // let mut real_papre:Vec<(usize,usize)>=Vec::new();

    for (dx, dy) in directions.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx < 0 || ny < 0 {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if nx >= grid.len() || ny >= grid[0].len() {
            continue;
        }

        if grid[nx][ny] == "@" {
            paper_count += 1;
            // real_papre.push((new_x as usize,new_y as usize));
        }

        if paper_count >= 4 {
            return false;
            // real_papre.push((x,y));
        }
    }

    // real_papre;
    paper_count < 4
}
