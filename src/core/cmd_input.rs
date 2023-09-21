#[allow(dead_code)]
pub fn get_arg_integer() -> Result<u128, String> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(
            "Please run the executable file with one additional argument.".to_string()
        );
    }
    match args[1].parse() {
        Ok(int) => Ok(int),
        Err(error) => Err(error.to_string()),
    }
}