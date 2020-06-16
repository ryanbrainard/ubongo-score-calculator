use std::env;
use ubongo_score_calculator::ops;

fn main() {
    let args = env::args().skip(1);
    ops::run(args).map_or_else(|err| -> () {
        println!("ERROR: {:?}", err);
    }, |score| -> () {
        println!("SCORE: {:?}", score);
    })
}
