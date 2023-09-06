mod prime_factor;
use prime_factor::prime_factorization;

mod input;
use input::get_input_integer;

mod strings;
use strings::get_equation_string;

fn main() {
    let num: u128;
    match get_input_integer() {
        Ok(int) => num = int,
        Err(error) => {
            println!("[ERROR] {}", error);
            return;
        },
    }
    let prime_map = prime_factorization(num);
    let equation = get_equation_string(&prime_map, num);
    println!("{}", equation);
}
