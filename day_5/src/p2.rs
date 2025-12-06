// use std::collections::HashSet;

// pub fn p2() {
//     let raw_lines: Vec<&str> = include_str!("../data/input.txt").lines().collect();

//     let mut numbers_to_check: Vec<u128> = Vec::new();
//     let mut ranges_only: Vec<(u128, u128)> = Vec::new();

//     // Parse input
//     for line in &raw_lines {
//         if line.contains('-') {
//             let nums: Vec<u128> = line
//                 .split('-')
//                 .filter_map(|x| x.parse::<u128>().ok())
//                 .collect();
//             if nums.len() == 2 {
//                 ranges_only.push((nums[0], nums[1]));
//             }
//         } else if let Ok(num) = line.trim().parse::<u128>() {
//             numbers_to_check.push(num);
//         }
//     }

//     // Use HashSet to track which ranges contain each number
//     // let mut acceptable_ranges = HashSet::new();
//     // let mut good_rangess:Vec<(u128,u128)>=Vec::new();

//     // for num in numbers_to_check {
//     //     for &(lower, upper) in &ranges_only {
//     //         if num >= lower && num <= upper {
//     //             // Instead of storing the entire range, just add the numbers we need
//     //             // acceptable_ranges.insert(num);
//     //             good_rangess.push((lower,upper));
//     //             break;
//     //         }
//     //     }
//     // }

//     //count
//     let mut temp_numbers:HashSet<u128>=HashSet::new();

//     for (lower,upper) in ranges_only{
//         for i in lower..=upper{
//             temp_numbers.insert(i);
//         }
//     } 

  

//     println!("the total count is {}", temp_numbers.len());
// }

pub fn p2() {
    let raw_lines: Vec<&str> = include_str!("../data/input.txt").lines().collect();

    let mut ranges: Vec<(u128, u128)> = Vec::new();

    // 1. Parse ONLY the ranges (ignore single numbers as per prompt)
    for line in raw_lines {
        if line.contains('-') {
            let nums: Vec<u128> = line
                .split('-')
                .filter_map(|x| x.trim().parse::<u128>().ok())
                .collect();
            if nums.len() == 2 {
                // Ensure ranges are stored as (min, max) just in case input is flipped
                let start = std::cmp::min(nums[0], nums[1]);
                let end = std::cmp::max(nums[0], nums[1]);
                ranges.push((start, end));
            }
        }
    }

    // 2. Sort ranges by start position
    // This is crucial for the merging logic to work
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    // 3. Merge overlapping intervals
    let mut merged_ranges: Vec<(u128, u128)> = Vec::new();
    
    if let Some(first) = ranges.first() {
        let mut current_start = first.0;
        let mut current_end = first.1;

        println!("first is {}",current_start);
        println!("second is {}",current_end);

        for &(next_start, next_end) in ranges.iter().skip(1) {
            // Check if the next range starts before (or exactly when) the current one ends.
            // Note: If the puzzle considers 3-5 and 6-10 as continuous, change to:
            // if next_start <= current_end + 1
            if next_start <= current_end + 1 { 
                // It overlaps or touches, so extends the current window
                current_end = std::cmp::max(current_end, next_end);
            } else {
                // No overlap, push the completed range and start a new one
                merged_ranges.push((current_start, current_end));
                current_start = next_start;
                current_end = next_end;
            }
        }
        // Don't forget to push the final range
        merged_ranges.push((current_start, current_end));
    }

    // 4. Calculate total count
    // Formula for inclusive range count: (Max - Min) + 1
    let mut total_count: u128 = 0;
    for (start, end) in merged_ranges {
        total_count += (end - start) + 1;
    }

    println!("The total fresh ingredient count is {}", total_count);
}