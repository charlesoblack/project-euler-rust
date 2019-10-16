fn main() {
    println!("Hello, world!");
    let mut curr: i32 = 1;
    let divisors: Vec<i32> = (1..20).collect();
    let result: i32 = loop {
        curr += 1;
        let divisible: bool = divisors.iter()
                                      .all(|&x| curr % x == 0);
        if divisible == true {
            break curr
        }
    };

    println!("{}", result)
}
