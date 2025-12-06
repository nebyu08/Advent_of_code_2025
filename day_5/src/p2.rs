use std::collections::HashSet;

pub fn p2() {
    let raw_lines: Vec<&str> = include_str!("../data/input.txt").lines().collect();

    let mut numbers_to_check: Vec<u128> = Vec::new();
    let mut ranges_only: Vec<(u128, u128)> = Vec::new();

    // Parse input
    for line in &raw_lines {
        if line.contains('-') {
            let nums: Vec<u128> = line
                .split('-')
                .filter_map(|x| x.parse::<u128>().ok())
                .collect();
            if nums.len() == 2 {
                ranges_only.push((nums[0], nums[1]));
            }
        } else if let Ok(num) = line.trim().parse::<u128>() {
            numbers_to_check.push(num);
        }
    }

    // Use HashSet to track which ranges contain each number
    // let mut acceptable_ranges = HashSet::new();
    // let mut good_rangess:Vec<(u128,u128)>=Vec::new();

    // for num in numbers_to_check {
    //     for &(lower, upper) in &ranges_only {
    //         if num >= lower && num <= upper {
    //             // Instead of storing the entire range, just add the numbers we need
    //             // acceptable_ranges.insert(num);
    //             good_rangess.push((lower,upper));
    //             break;
    //         }
    //     }
    // }

    //count
    let mut temp_numbers:HashSet<u128>=HashSet::new();

    for (lower,upper) in ranges_only{
        for i in lower..=upper{
            temp_numbers.insert(i);
        }
    } 

  

    println!("the total count is {}", temp_numbers.len());
}