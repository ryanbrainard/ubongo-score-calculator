use std::env;
mod lib;

fn main() {
    let args = env::args().skip(1);
    let score = lib::run(args);
    println!("SCORE: {:?}", score);
}
