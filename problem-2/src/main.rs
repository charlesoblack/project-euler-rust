fn main() {
    println!("Hello, world!");

    let mut num_2: i32;
    let mut num_1: i32 = 1;
    let mut num_0: i32 = 1;
    let mut sum: i32 = 0;
    let limit: i32 = 4 * 10_i32.pow(6);

    while num_0 < limit {
        if num_0 % 2 == 0 {
            sum += num_0;
        }
        num_2 = num_1;
        num_1 = num_0;
        num_0 += num_2;
    }
    println!("{}", sum);
}
