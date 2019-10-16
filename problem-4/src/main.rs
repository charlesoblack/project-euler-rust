extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("Hello, world!");

    let mut largest = 1;

    for num in 100..999 {
        for num_2 in 100..999 {
            let product = num * num_2;
            let string = product.to_string();
            let reversed: String = UnicodeSegmentation::graphemes(&*string, true)
                                    .rev()
                                    .collect();


            if (string == reversed) & (product > largest) {
                largest = product;
            }
        }
    }
    println!("{}", largest);
}
