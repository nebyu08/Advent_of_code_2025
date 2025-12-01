use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("Unable to open file");
    let reader: BufReader<File> = BufReader::new(file);
    let mut count_zero = 0;
    let mut start_point;
    let mut result_cal;

    start_point = 50;
    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result?;

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
    println!("The number of times we reach 0 is: {}", count_zero);
    Ok(())
}
