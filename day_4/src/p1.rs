use std::io::{BufRead, BufReader};
use std::fs::File;


pub fn p1(){
    let file=File::open("data/input.txt").unwrap();
    let reader=BufReader::new(file);
    let mut grid: Vec<Vec<String>>=Vec::new();
    let mut total_count=0;

    for line_result in reader.lines(){
        let line=line_result.unwrap();
        let row:Vec<String>=line.chars().map(|c| c.to_string()).collect();
        grid.push(row);
    }

    // loop through the grid and print each element with its coordinates
    for (i, row) in grid.iter().enumerate(){
        for (j, _) in row.iter().enumerate(){
            // println!("grid[{}][{}] = {}", i, j, value);
            if have_less_than_4_papers_fn(&grid, i,j){
                total_count+=1;
            }

        }
    }


    println!("Total count of positions with less than 4 papers around: {}", total_count);
}

fn have_less_than_4_papers_fn(grid: &Vec<Vec<String>>, x: usize, y: usize) -> bool {

    // let direction_up: (usize, usize)=(x-1, y);
    // let direction_down=(x+1, y);
    // let direction_left=(x, y-1);
    // let direction_right=(x, y+1);

    // let directions = [direction_up, direction_down, direction_left, direction_right];
    let directions: [(isize, isize); 4]=[(0isize,-1),(0,1),(-1,0),(1,0)];
    let mut paper_count = 0;
    // let mut real_papre:Vec<(usize,usize)>=Vec::new();


    for (dx, dy) in directions.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx <0 || ny <0 {
            continue;
        }

        let nx= nx as usize;
        let ny= ny as usize;

        if nx >= grid.len()  || ny >= grid[0].len() {
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