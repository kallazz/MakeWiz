pub fn parse_arguments(args: &Vec<String>) -> Result<&str, &str> {
    if args.len() == 1 {
        return Ok("main");
    }
    //Flags will be added later

    let executable_name = &args[1];

    Ok(executable_name)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_arguments() {
        let args: Vec<String> = vec![String::from("target/debug/genmake")];
        assert_eq!(Err("Not enough arguments"), parse_arguments(&args));
    }

    #[test]
    fn one_argument() {
        let args: Vec<String> = vec![String::from("target/debug/genmake"), String::from("filename")];
        assert_eq!(Ok("filename"), parse_arguments(&args));
    }
}