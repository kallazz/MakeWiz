pub fn parse_arguments(args: &Vec<String>) -> Result<(&str, &str), &str> {
    let mut executable = "main";
    let mut compiler = "g++";

    if args.len() >= 2 {
        executable = &args[1];
    }
    if args.len() == 3 {
        compiler = &args[2];
    }
    if args.len() > 3 {
        return Err("Too many arguments");
    }

    Ok((executable, compiler))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_argument() {
        let args: Vec<String> = vec![String::from("target/debug/genmake"), 
            String::from("executable")];
        assert_eq!(Ok(("executable", "g++")), parse_arguments(&args));
    }

    #[test]
    fn two_arguments() {
        let args: Vec<String> = vec![String::from("target/debug/genmake"), 
            String::from("executable"), String::from("compiler")];
        assert_eq!(Ok(("executable", "compiler")), parse_arguments(&args));
    }
}
