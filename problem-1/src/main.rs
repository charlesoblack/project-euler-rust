extern crate time;
use time::PreciseTime;

fn main() {
    println!("Hello, world!");

    let start = PreciseTime::now();
    // whatever you want to do

    let mut sum = 0;
    let max_num = 1000;

    for number in 1..max_num {
        if (number % 3 == 0) | (number % 5 == 0) {
            sum += number;
        }
    }
    println!("{}", sum);
    let end = PreciseTime::now();
    println!("{} seconds for whatever you did.", start.to(end));
}
