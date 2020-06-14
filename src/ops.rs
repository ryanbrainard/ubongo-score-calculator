pub fn run<T: Iterator<Item = String>>(args: T) -> u32 {
    Gems::from_args(args).calculate_score()
}

#[derive(Debug)]
pub struct Gems {
    yellow: u32,
    green: u32,
    blue: u32,
    red: u32,
}

impl Gems {
    pub fn from_args<T: Iterator<Item = String>>(mut args: T) -> Self {
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

    pub fn calculate_score(&self) -> u32 {
        (self.yellow * 1) + (self.green * 2) + (self.green * 3) * (self.red * 4)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        // TODO: can i simplify this conversion or allow run() to just accept any kind of string iterator?
        let args = vec!["2", "3", "4", "5"].into_iter().map(|s|s.to_string());
        assert_eq!(run(args), 188);
    }
}
