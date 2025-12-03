use std::io::{BufRead, BufReader};
use std::fs::File;
use std::vec;

pub fn p1(){
    // let file=Path::new("data/input.txt");
    let file=File::open("data/input.txt").unwrap();
    let reader=BufReader::new(file);
    let mut largest_pars:Vec<i32>=vec![];
    // let total_sum:i32=0;

    for line_result in reader.lines() {
        let line=line_result.unwrap();
        if line.is_empty(){
            continue;
        }

        let mut largest=0;
        for i in 0..line.len()-1{
            for j in i+1..line.len(){
                let first_digit=line.chars().nth(i).unwrap();
                let second_digit=line.chars().nth(j).unwrap();
                let current_pair_str=format!("{}{}",first_digit,second_digit);
                let current_pair=current_pair_str.parse::<i32>().unwrap();

                if current_pair>largest{
                    largest=current_pair;
                }
            }
            // let two_digits_str=&line[i..i+1];
            // let current_pair=two_digits_str.parse::<i32>().unwrap();
            
            // if current_pair>largest{
            //     largest=current_pair;
            // }
        }

        largest_pars.push(largest);
      
    }

    let total_sum: i32 = largest_pars.iter().sum();
    println!("The total sum of largest pairs is: {}", total_sum);

}