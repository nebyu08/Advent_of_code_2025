use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug,Clone,Copy)]
enum Operations{
    Add,
    Mull
}

impl Operations {
    fn apply(&self,a:f64,b:f64)->f64{
        match self{
            Operations::Add => a+b,
            Operations::Mull=>a*b,
            // _=>None,
        }
    }

}

fn parse_op(s:&str)->Option<Operations>{
    match s{
        "+"=> Some(Operations::Add),
        "*" => Some(Operations::Mull),
        _=>None
    }
}

// pub fn p1(){
//     let file=File::open("data/input.txt").unwrap();
//     let reader=BufReader::new(file);

//     let mut vector_numbers:Vec<Vec<f64>>=Vec::new();   
//     let mut vector_strings:Vec<String>=Vec::new();

//     for line in reader.lines(){
//         let line=line.unwrap();

//         let cols:Vec<&str>=line.split_whitespace().collect();
//         let maybe_numbers: Result<Vec<f64>, _>=cols.iter().map(|c| c.parse::<f64>()).collect();

//         match maybe_numbers{
//             Ok(nums)=> vector_numbers.push(nums),
//             Err(_)=>vector_strings.push(cols.iter().map(|s| s.to_string()).collect()),
//         }
//     }



//     let rows=vector_numbers.len();
//     let cols=vector_numbers[0].len();

//     println!("row is {}",rows);
//     println!("cols is {}",cols);

//     // let column_operations:Vec<u128>=Vec::new();

//     for i in 0..cols{
//         for row in 0..rows{
//             let op=parse_op(vector_strings[i].as_str());
//             let temp_result=
//         }
//     }

// }

pub fn p1() {
    let file = File::open("data/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut vector_numbers: Vec<Vec<f64>> = Vec::new();   
    let mut vector_strings: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let cols: Vec<&str> = line.split_whitespace().collect();
        let maybe_numbers: Result<Vec<f64>, _> =
            cols.iter().map(|c| c.parse::<f64>()).collect();

        match maybe_numbers {
            Ok(nums) => vector_numbers.push(nums),
            Err(_) => vector_strings.push(cols.join(" ")),
        }
    }

    let rows = vector_numbers.len();
    let cols = vector_numbers[0].len();

    // --- Extract column operators ---
    let mut column_ops: Vec<Operations> = Vec::new();
    for op_line in &vector_strings {
        let ops: Vec<&str> = op_line.split_whitespace().collect();
        for (i, s) in ops.iter().enumerate() {
            if column_ops.len() <= i {
                column_ops.push(parse_op(s).unwrap());
            }
        }
    }

    // --- Apply operations column-wise ---
    let mut column_results = Vec::new();
    for col in 0..cols {
        let op = column_ops[col];
        let mut acc = vector_numbers[0][col];

        for row in 1..rows {
            acc = op.apply(acc, vector_numbers[row][col]);
        }

        column_results.push(acc);
    }

    let mut sum: f64=0.0;

    // --- Print results ---
    for (_col, result) in column_results.iter().enumerate() {
        sum+=result;
        // println!("Column {} result = {}", col, result);
    }

    println!("final result is {}",sum);
}
