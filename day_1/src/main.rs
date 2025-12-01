mod day1;

fn main(){
    // day1::get_zero_end();
    let count=day1::total_amount_of_zero_occured();
    let count_1=day1::total_count_of_zero();
    println!("Total passes through zero: {}",count);
    println!("Total passes through zero: {}",count_1);
}
