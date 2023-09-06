use std::collections::HashMap;

pub fn get_equation_string(prime_map: &HashMap<u128, u128>, num: u128) -> String {
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