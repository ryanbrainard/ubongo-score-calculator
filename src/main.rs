use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1);
    ubongo_score_calculator::ops::run(args).map(|score| -> () {
        println!("SCORE: {:?}", score);
    })
}
