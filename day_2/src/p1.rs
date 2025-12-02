use std::fs::File;
use std::io::{self, BufRead};

pub fn p1() {
    let file = File::open("data/input.txt").expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let mut repition: Vec<i64> = vec![];
    let mut total_sum: i64 = 0;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        for pair in line.split(',') {
            let bounds: Vec<&str> = pair.split('-').collect();
            let start: i64 = bounds[0].parse().expect("Unable to parse start");
            let end: i64 = bounds[1].parse().expect("Unable to parse end");

            for num in start..=end {
                let str_num = num.to_string();
                if str_num.len() % 2 != 0 {
                    continue;
                }

                let (left_num, right_num) = split_into_halves(str_num.as_str());
                if left_num == right_num {
                    println!("Found repitition: {}", num);
                    repition.push(num);
                }
            }
        }

        for i in &repition {
            total_sum += i;
        }

        println!("Total sum of repititions: {}", total_sum);

        // println!("{:?}", pairs);
    }
}

fn split_into_halves(s: &str) -> (&str, &str) {
    let mid = s.len() / 2;
    s.split_at(mid)
}

pub fn p2() {
    let file = File::open("data/input.txt").expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let mut repition: Vec<i64> = vec![];
    let mut total_sum: i64 = 0;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        for pair in line.split(',') {
            let bounds: Vec<&str> = pair.split('-').collect();
            let start: i64 = bounds[0].parse().expect("Unable to parse start");
            let end: i64 = bounds[1].parse().expect("Unable to parse end");

            for num in start..=end {
                let str_num = num.to_string();
                if is_repeating(&str_num) {
                    // println!("Found repitition: {}", num);
                    repition.push(num);
                }
            }
        }

        for i in &repition {
            total_sum += i;
        }

        println!("Total sum of repititions: {}", total_sum);

        // println!("{:?}", pairs);
    }
}

fn is_repeating(s: &str) -> bool {
    let len = s.len();
        for size in 1..=(len / 2) {
            if len % size != 0 {
                continue;
            }
            let part = s[0..size].to_string();
            let repeats = len / size;
            let constructed = part.repeat(repeats);

            if constructed == s {
                return true;
            }
        }

    false
}
