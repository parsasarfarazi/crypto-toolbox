pub fn modpow(number: u64, exponent: u64, modulo: u64) -> u64 {
    let binary_expansion = get_binary_digits(exponent);
    println!("binary_expansion is : {binary_expansion:?}");

    let mut inter_numbers: Vec<u64> = Vec::new();
    inter_numbers.push(number % modulo);
    for i in 1..binary_expansion.len() {

        inter_numbers.push((inter_numbers[i-1].pow(2)) % modulo);
    }
    println!("inter_numebrs is : {inter_numbers:?}");

    let mut result: u64 = 1;
    for i in 0..binary_expansion.len() {
        result *= inter_numbers[i].pow(binary_expansion[i] as u32);
        result = result % modulo
    }
    result
}

pub fn get_binary_digits(number: u64) -> Vec<u8> {
    let mut result = Vec::new();
    let mut n = number;
    if n == 0 {
        return vec![0];
    }
    while n > 0 {
        result.push((n % 2) as u8);
        n /= 2;
    }
    result
}

#[cfg(test)]
mod test {
    use crate::utils::arithmatic;

    use super::*;

    #[test]
    fn test_modpow() {
        let result = arithmatic::modpow(7, 16, 17);
        dbg!(result);
        assert_eq!(result, 1)
    }
}
