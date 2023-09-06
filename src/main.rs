use std::collections::HashMap;
use std::num::ParseIntError;

mod prime_factor;
use prime_factor::prime_factorization;

fn get_equation_string(prime_map: &HashMap<u128, u128>, num: u128) -> String {
    let mut primes = prime_map.keys()
                              .map(|num| *num)
                              .collect::<Vec<u128>>();
    primes.sort();
    let equation = {
        primes.iter().rev().enumerate().rev()
              .fold(String::new(), |acc, (i, prime)| {
                let term = {
                    let freq = *prime_map.get(&prime).unwrap();
                    if freq == 1 {
                        prime.to_string()
                    } else {
                        prime.to_string() + "^" + &freq.to_string()
                    }
                };
                acc + &term + if i == 0 { "" } else { " * " }
              }) + " = " + &num.to_string()
    };
    equation
}

fn get_input_integer() -> Result<u128, ParseIntError> {
    let mut line = String::new();
    println!("Number:");
    let _ = std::io::stdin().read_line(&mut line);
    line.trim().parse()
}

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
