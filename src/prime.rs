use std::collections::btree_map::Range;

use crate::{prng, utils};
pub fn gen_prime() -> u128 {
    //gen a random odd number
    // check for divisibility on first 100 primes
    // miller robin
    // output
    loop {
        let candidate = prng::odd_new();
        if check_first_n_primes_divisibility(candidate) {
            if !is_composite(candidate) {
                return candidate;
            }
        }

        // miller robin to be added
    }
}

fn is_composite(n: u128) -> bool {
    let res = utils::mod_pow(n, 2, n - 1);
    if res != 1 { true } else { false }
}

fn check_first_n_primes_divisibility(candidate: u128) -> bool {
    let primes = first_nth_prime(100);
    for p in primes {
        if candidate % p == 0 {
            return false;
        }
    }
    true
}

fn first_nth_prime(n: u128) -> Vec<u128> {
    let mut primes: Vec<u128> = Vec::new();
    primes.push(2);

    let mut num = 2;

    loop {
        num += 1;
        let mut is_prime = true;
        for p in &primes {
            if num % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(num);
        }
        if primes.len() == n as usize {
            break;
        }
    }

    primes
}

fn miller_rabin_core(n: u128, a: u128) -> bool {
    // initial check
    if n % 2 == 0 {
        return true;
    }
    if utils::gcd(n, a) != 1 {
        return true;
    }

    // n - 1 = 2^k.q
    let mut q = n;
    let mut k = 0;
    loop {
        if n % 2 == 0 {
            q = q / 2;
            k += 1
        } else {
            break;
        }
    }

    let mut a = utils::mod_pow(n, a, q);

    if a == 1 {
        return false;
    }

    for _ in 0..k {
        if a == 1 {
            return false;
        } else {
            a = utils::mod_pow(n, a, 2)
        }
    }
    false
}
