use std::num::ParseIntError;

pub fn get_input_integer() -> Result<u128, ParseIntError> {
    let mut line = String::new();
    println!("Number:");
    let _ = std::io::stdin().read_line(&mut line);
    line.trim().parse()
}
