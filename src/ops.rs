use std::error::Error;

pub fn run<T: Iterator<Item = String>>(args: T) -> Result<u32, Box<dyn Error>> {
    Gems::from_args(args).map(|gems| gems.calculate_score())
}

#[derive(Debug)]
pub struct Gems {
    yellow: u32,
    green: u32,
    blue: u32,
    red: u32,
}

impl Gems {
    pub fn from_args<T: Iterator<Item = String>>(mut args: T) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            yellow: Self::from_arg("yellow", args.next())?,
            green: Self::from_arg("green", args.next())?,
            blue: Self::from_arg("blue", args.next())?,
            red: Self::from_arg("red", args.next())?,
        })
    }

    fn from_arg(name: &str, arg_str_opt: Option<String>) -> Result<u32, Box<dyn Error>> {
        match arg_str_opt {
            Some(arg_str) => {
                match arg_str.parse::<u32>() {
                    Ok(val) => Ok(val),
                    Err(err) => Err(format!("arg parse error: {}", err).into()),
                }
            }
            None => Err(format!("no `{}` arg found", name).into()),
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
    fn test_run_ok() {
        // TODO: can i simplify this conversion or allow run() to just accept any kind of string iterator?
        let args = vec!["2", "3", "4", "5"].into_iter().map(|s|s.to_string());
        let result = run(args);
        // TODO: ok to upwrap in a test?
        assert_eq!(result.unwrap(), 188);
    }

    #[test]
    fn test_run_missing_arg() {
        // TODO: can i simplify this conversion or allow run() to just accept any kind of string iterator?
        let args = vec!["2", "3", "4"].into_iter().map(|s|s.to_string());
        let result = run(args);
        // TODO: ok to upwrap in a test?
        assert_eq!(result.unwrap_err().to_string(), "no `red` arg found");
    }
}
