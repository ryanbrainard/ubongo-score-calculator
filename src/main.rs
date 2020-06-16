use std::env;
use std::error::Error;
use ubongo_score_calculator::ops;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1);
    ops::run(args).map(|score| -> () {
        println!("SCORE: {:?}", score);
    })
}
