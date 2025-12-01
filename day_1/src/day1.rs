use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::ptr::read;



pub fn total_amount_of_zero_occured() -> i32 {
    let file = File::open("data/input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut current_position: i32 = 50;
    let mut zero_count: i32 = 0;

    for line in reader.lines().flatten() {
        if line.is_empty() {
            continue;
        }

        let dir = match line.chars().next() {
            Some(d) => d,
            None => continue,
        };

        let value: i32 = match line[1..].trim().parse() {
            Ok(v) if v >= 0 => v,
            _ => continue,
        };

        let delta = match dir {
            'R' => 1,
            'L' => -1,
            _ => continue,
        };

        // Simulate exactly what the correct solution does: one step at a time
        for _ in 0..value {
            current_position += delta;
            current_position = current_position.rem_euclid(100);
            if current_position == 0 {
                zero_count += 1;
            }
        }
    }

    zero_count
}

// pub fn get_pass_0() -> i32 {
//     let file = File::open("data/input.txt").expect("Unable to open file");
//     let reader = BufReader::new(file);

//     let mut current_position = 50;
//     let mut zero_count = 0;

//     for line in reader.lines().flatten() {
//         if line.is_empty() {
//             continue;
//         }

//         let dir = line.chars().next().unwrap();
//         // Parse the magnitude of the move
//         let value = line[1..].parse::<i32>().unwrap();

//         // Determine the step direction
//         let step = match dir {
//             'R' => 1,
//             'L' => -1,
//             _ => continue,
//         };

//         // ALGORITHM FIX: Simulate 1 step at a time (Flatten logic)
//         // The reference code converts "Right 5" into "Right 1" five times.
//         // We must loop 'value' times and check 0 at every increment.
//         for _ in 0..value {
//             // Use rem_euclid to handle wrapping 0..99 correctly for negative numbers
//             current_position = (current_position + step).rem_euclid(100);

//             if current_position == 0 {
//                 zero_count += 1;
//             }
//         }
//     }

//     zero_count
// }

// pub fn get_pass_0()->i32{
//     let file = File::open("data/input.txt").expect("Unable to open file");
//     let reader: BufReader<File> = BufReader::new(file);
//     let mut current_position = 50;
//     let mut pass_zero = 0;
//     let mut count_zero = 0;

//     for line in reader.lines().flatten() {
//         if line.is_empty() {
//             continue;
//         }

//         let dir = line.chars().next().unwrap();
//         let value = line[1..].parse::<i32>().unwrap();

//         let delta = match dir {
//             'R' => value,
//             'L' => -value,
//             _ => continue,
//         };

//         let raw = current_position + delta;

//         // Count loops past zero:
//         pass_zero += raw.div_euclid(100).abs();

//         // Wrap position to 0..99
//         current_position = raw.rem_euclid(100);

//         if current_position == 0 {
//             count_zero += 1;
//         }
//     }

//      pass_zero + count_zero

    

    

//     // return pass_zero;
//     // return pass_zero+count_zero;

// }

pub fn total_count_of_zero()->i32{
    let file = File::open("input.txt").expect("Unable to open file");
    let reader: BufReader<File> = BufReader::new(file);
    let mut count_zero = 0;
    let mut start_point;
    let mut result_cal;

    start_point = 50;
    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result.unwrap();

        if line.is_empty() {
            continue;
        }

        let first_char = line.chars().next().unwrap();
        let rest_of_line = line.get(1..).unwrap().parse::<i32>().unwrap();

        match first_char {
            'R' => {
                result_cal = start_point + rest_of_line;
                // start_point = result_cal; 

                while result_cal<0 {
                    result_cal += 100;
                }
                while result_cal>99 {
                    result_cal -= 100;
                }
                
                start_point = result_cal;
                if result_cal == 0 {
                    count_zero += 1;
                }
                
            }
            'L' => {
                result_cal = start_point - rest_of_line; // 0 -1

                while result_cal<0 {
                    result_cal += 100;
                }
                while result_cal>99 {
                    result_cal -= 100;
                }
                
                start_point = result_cal;

                if result_cal == 0 {
                    count_zero += 1;
                }
                // start_point = result_cal;
            }
            _ => {
                println!("Invalid direction at line {}", line_number + 1);
                continue;
            }
        }
    }

    count_zero
}