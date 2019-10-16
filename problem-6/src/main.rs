fn main() {
    println!("Hello, world!");

    let limit = 100;
/*
    let sum_of_squares: i32 = (1_i32..limit + 1).map(|x| x.pow(2))
                                   .sum();*/
    let sum_of_squares: i32 = (1_i32..limit + 1).fold(0, |acc, x| acc + x.pow(2));
    let square_of_sums: i32 = (1_i32..limit + 1)
                                .fold(0, |acc, x| acc + x)
                                .pow(2);
    println!("{}", square_of_sums - sum_of_squares);
}
