/// Calculates the greatest common divisors of a vector of numbers using the euclidian algorithm
/// #### Arguments
/// * `numbers` - A vector of unsigned 32-bit integer
/// #### Returns
/// * `0` if the input is empty
/// * the gcd of the numbers
/// #### Examples
/// ```
/// use crypto_toolbox::utils::gcd;
///
/// let numbers = vec![12, 18, 24];
/// assert_eq!(gcd::gcd(numbers), 6);
///
/// let empty: Vec<u32> = vec![];
/// assert_eq!(gcd::gcd(empty), 0);
/// ```
pub fn gcd(numbers: Vec<u32>) -> u32 {
    if numbers.is_empty() {
        return 0;
    }
    if numbers.len() == 1 {
        return numbers[0];
    }
    let mut result: u32 = numbers[0];
    for &number in numbers.iter().skip(1) {
        result = gcd_of_two(result, number);
        if result == 1 {
            return 1;
        }
    }
    result
}

/// core logic of the gcd eulidian algorithm for only two numbers
fn gcd_of_two(r0: u32, r1: u32) -> u32 {
    if r1 == 0 { r0 } else { gcd_of_two(r1, r0 % r1) }
}



#[cfg(test)]
mod test {
    use super::gcd;

    #[test]
    fn test_gcd() {
        let numbers: Vec<u32> = vec![5, 10, 15];
        let g = gcd(numbers);
        assert_eq!(g, 5)
    }
}
