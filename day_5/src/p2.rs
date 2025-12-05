use std::collections::HashSet;

pub fn p2() {
    let raw_lines: Vec<&str> = include_str!("../data/input.txt").lines().collect();

    let mut numbers_to_check: Vec<u128> = Vec::new();
    // let mut count_exists:u128=0;
    let mut data: Vec<(u128, u128)> = Vec::new();
    let mut total_numers: Vec<u128> = Vec::new();

    let ranges_only: Vec<(u128, u128)> = raw_lines
        .iter()
        .filter_map(|line| {
            if !line.contains('-') {
                return None;
            }

            let nums = line
                .split('-')
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<u128>>();

            (nums.len() == 2).then(|| (nums[0], nums[1]))
        })
        .collect();

    // println!("the ranges only are {:?}",ranges_only);

    for line in &raw_lines {
        if line.contains('-') {
            continue;
        }
        if let Ok(num) = line.trim().parse::<u128>() {
            numbers_to_check.push(num);
        }
    }

    // println!("the individual value are {:?}",numbers_to_check);

    for i in numbers_to_check {
        for &j in &ranges_only {
            let (temp_lower, temp_upper) = j;
            let range_inclusive = temp_lower..=temp_upper;
            if range_inclusive.contains(&i) {
                data.push(j);
                // count_exists+=1;
                break;
            }
        }
    }

    for (lower, upper) in data {
        for i in lower..=upper {
            total_numers.push(i);
        }
        // let temp_count: u128=temp.count().try_into().unwrap();
        // count_exists+=temp_count;
    }

    // let counting=total_numers
    let number_set:HashSet<u128>=total_numers.into_iter().collect();

    println!("the total count is {}",number_set.len());
}
