use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main()->io::Result<()> {
    let file=File::open("input.txt").expect("Unable to open file");
    let reader: BufReader<File>=BufReader::new(file);
    let mut count_zero=0;
    let start_point=50;
    let mut result_cal=0;

    for (line_number,line_result) in reader.lines().enumerate() {

        let line=line_result?;

        let first_char=line.chars().next().unwrap();
        // let rest_of_line=line.chars().skip(1).collect::<String>().parse::<i32>().unwrap();
        let rest_of_line=line.get(1..).unwrap().parse::<i32>().unwrap();


        match first_char {
            'R' => {
                result_cal=start_point+rest_of_line;
                while result_cal>100 {
                    result_cal-=100;
                    if result_cal==0 {
                        count_zero+=1;
                    }
                }
            },
            'L' => {
                result_cal=start_point-rest_of_line;
                while result_cal<0 {
                    result_cal+=100;
                    if result_cal==0 {
                        count_zero+=1;
                }
            }
        },
            _ => {
                println!("Invalid direction at line {}", line_number + 1);
                continue;
            }
         
    // }

    
        }
    }
    println!("The number of times we reach 0 is: {}", count_zero);
    Ok(())
}
