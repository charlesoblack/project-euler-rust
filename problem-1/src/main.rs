fn main() {
    println!("Hello, world!");

    let mut sum = 0;
    let max_num = 1000;

    for number in 1..max_num {
        if (number % 3 == 0) | (number % 5 == 0) {
            sum += number;
        }
    }
    println!("{}", sum)
}
