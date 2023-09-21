mod core;
#[allow(unused_imports)]
use core::{ get_arg_integer, get_input_integer, prime_factorization, get_equation_string };

fn main() {
    let num: u128;
    match get_arg_integer() {
        Ok(int) => num = int,
        Err(error) => {
            return println!("[ERROR] {}", error);
        }
    }
    let prime_map = prime_factorization(num);
    let equation = get_equation_string(&prime_map, num);
    println!("{}", equation);
}
