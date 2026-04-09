// Psudo-Random Number Generator

use std::time::{SystemTime, UNIX_EPOCH};

const M: u128 = 2_u128.pow(32);
const A: u128 = 1103515245;
const C: u128 = 12345;
const N: usize = 20;

pub fn new() -> u128 {
    let seed = get_time();

    let mut random_values: Vec<u128> = Vec::new();
    random_values.push((seed * A + C) % M);

    for i in 1..=N {
        random_values.push((random_values[i as usize - 1] * A + C) % M)
    }

    return random_values[N as usize - 1];
}

pub fn odd_new() -> u128 {
    let candidate = new();

    if candidate % 2 == 1 {
        return candidate;
    } else {
        return candidate + 1;
    }
}

fn get_time() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u128;
}
