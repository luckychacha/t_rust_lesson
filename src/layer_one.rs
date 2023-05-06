pub mod layer_two;

pub fn print_char() {
    println!("layer one start.");
    // for letter in ('A'..='z').rev() {
    //     println!("{letter}");
    // }
    for letter in ('a'..='z').chain('A'..='Z') {
        println!("{letter}");
    }
    println!("layer one end.");
}
