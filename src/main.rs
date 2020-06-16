use std::env;

use ubongo_score_calculator::ops;

fn main() {
    let args = env::args().skip(1);
    match ops::run(args) {
        Ok(score) => println!("SCORE: {:?}", score),
        Err(err) => println!("ERROR: {:?}", err),
    }
}
