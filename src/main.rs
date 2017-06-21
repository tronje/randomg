extern crate randomg;


fn main() {
    // seed with a random number
    // (I rolled a dice, I promise!)
    let mut gen = randomg::get_generator(4);

    // generate some gibberish
    let gibberish = randomg::generate_string(100, &mut gen);
    let more_gibberish = randomg::generate_string(100, &mut gen);

    println!("{}", gibberish);
    println!("{}", more_gibberish);
}
