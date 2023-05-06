pub fn print_char() {
    println!("layer two start.");
    // for letter in 'A'..='z' {
    //     println!("{letter}");
    // }

    for letter in ('A'..='Z').chain('a'..='z') {
        println!("{letter}");
    }
    println!("layer two end.");
}
