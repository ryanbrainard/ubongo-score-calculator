use std::env;

fn main() {
    let mut args = env::args();
    args.next(); // discard arg0
    let gems = Gems::from_args(args);
    let score = gems.calculate_score();
    println!("SCORE: {:?}", score);
}

#[derive(Debug)]
struct Gems {
    yellow: u32,
    green: u32,
    blue: u32,
    red: u32,
}

impl Gems {
    fn from_args(mut args: env::Args) -> Self {
        Self {
            yellow: Self::from_arg("yellow", args.next()),
            green: Self::from_arg("green", args.next()),
            blue: Self::from_arg("blue", args.next()),
            red: Self::from_arg("red", args.next()),
        }
    }

    fn from_arg(name: &str, arg_str_opt: Option<String>) -> u32 {
        match arg_str_opt {
            Some(arg_str) => {
                match arg_str.parse::<u32>() {
                    Ok(val) => val,
                    Err(err) => panic!(format!("arg parse error: {}", err)),
                }
            }
            None => panic!(format!("no `{}` arg found", name)),
        }
    }

    fn calculate_score(&self) -> u32 {
        (self.yellow * 1) + (self.green * 2) + (self.green * 3) * (self.red * 4)
    }
}
