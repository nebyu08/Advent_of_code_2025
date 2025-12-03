use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn p2()->u64{
    let file=File::open("data/input.txt").unwrap();
    let reader=BufReader::new(file);
    let mut total_sum:u64=0;
    // let mut largest_pars:Vec<u128>=vec![];


    for line_result in reader.lines() {
        let line=line_result.unwrap();
        if line.is_empty(){
            continue;
        }

        let result=solve_line(&line);
        total_sum+=result;

      
    }

    // let total_sum: u128 = largest_pars.iter().sum();
    println!("The total sum of largest pairs is: {}", total_sum);

    total_sum

}

fn pick_max_subsequence_12(digits: &[u32]) -> Vec<u32> {
    let target = 12;
    let mut stack = Vec::with_capacity(target);

    // How many digits we are allowed to drop
    let mut drops = digits.len().saturating_sub(target);

    for &d in digits {
        // Pop smaller digits if allowed and current digit is bigger
        while let Some(&last) = stack.last() {
            if drops > 0 && last < d {
                stack.pop();
                drops -= 1;
            } else {
                break;
            }
        }
        stack.push(d);
    }

    // Keep only the first 12 digits
    stack.truncate(target);
    stack
}

fn digits_to_number(digits: &[u32]) -> u64 {
    digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

fn solve_line(line: &str) -> u64 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    let chosen = pick_max_subsequence_12(&digits);
    digits_to_number(&chosen)
}
