use std::env;

fn main() {
    let args = env::args().skip(1);
    let score = ubongo_score_calculator::ops::run(args);
    println!("SCORE: {:?}", score);
}
