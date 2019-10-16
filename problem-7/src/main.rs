fn is_prime(x: i32) -> bool {
    for num in 2..((x as f64).sqrt() as i32 + 1) {
        if x % num == 0 {
            return false
        }
    }
    true
}

fn main() {
    println!("Hello, world!");

    let mut counter: i16 = 0;
    let mut curr: i32 = 1;

    let result: i32 = loop {
        counter += 1;
        curr = (curr+1..).filter(|x| is_prime(*x)).next().unwrap();

        if counter == 10001 {
            break curr
        }
    };

    println!("{}", result);
}
