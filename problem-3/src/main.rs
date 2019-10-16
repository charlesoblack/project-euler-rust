extern crate time;
use time::PreciseTime;

fn is_prime(x: i64) -> bool {
    for num in 2..((x as f64).sqrt() as i64) {
        if x % num == 0 {
            return false
        }
    }
    true
}

fn main() {
    println!("Hello, world!");
    let start = PreciseTime::now();
    let goal_num: i64 = 600851475143;
    let sqrt_num: f64 = (goal_num as f64).sqrt();
    let mut curr: i64 = 2;
    let mut largest_prime: i64 = curr;

    while curr < (sqrt_num as i64) {
        curr += 1;
        if goal_num as i64 % curr == 0 {
            if is_prime(curr) {
               largest_prime = curr;
            }
        }
    }
    println!("{}", largest_prime);
    let end = PreciseTime::now();
    println!("{} seconds for whatever you did.", start.to(end));
}
