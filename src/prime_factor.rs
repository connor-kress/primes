use std::collections::HashMap;

pub fn is_prime(n: u128) -> bool {
    for i in 2..=(n as f64).sqrt() as u128 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

struct Prime {
  curr: u128,
  next: u128,
  trial1: u128,
  trial2: u128
}

impl Prime {
  pub fn new() -> Prime {
    Prime {
      curr: 2,
      next: 3,
      trial1: 5,
      trial2: 7
    }
  }
}

impl Iterator for Prime {
  type Item = u128;

  fn next(&mut self) -> Option<Self::Item> {
    let prime = self.curr;
    self.curr = self.next;
    loop {
      self.next = self.trial1;
      self.trial1 = self.trial2;
      self.trial2 = self.next+6;
      if is_prime(self.next) {
        break;
      }
    }
    Some(prime)
  }
}

fn add_prime(p: u128, map: &mut HashMap<u128, u128>) {
    if !map.contains_key(&p) {
        map.insert(p, 1);
    } else {
        *map.get_mut(&p).unwrap() += 1;
    }
}

pub fn prime_factorization(mut n: u128) -> HashMap<u128, u128> {
    if n < 2 {
        panic!("`prime_factorization` only accepts values of 2 or greater.");
    }
    let mut primes = HashMap::<u128, u128>::new();
    let mut current_prime = Prime::new();
    loop {
        if n == 1 {
            break;
        }
        loop {
            let p = current_prime.curr;
            if p > n {
                break;
            }
            if n % p != 0 {
                current_prime.next();
                continue;
            }
            // println!("{n}/{p}");
            add_prime(p, &mut primes);
            n /= p;
            break;
        }
    }
    primes
}