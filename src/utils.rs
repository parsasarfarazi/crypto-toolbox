use crypto_bigint::{self, Integer, U1024, U2048, U16384, Zero};

type Pint = U1024;
/// Calculates the greatest common divisors of a vector of numbers using the euclidian algorithm
/// #### Arguments
/// * `numbers` - A reference to a array of BigUints
/// #### Returns
/// * `0` if the input is empty
/// * the gcd of the numbers
/// #### Examples
/// ```
/// use crypto_toolbox::utils::gcd;
///
/// let numbers = vec![12, 18, 24];
/// assert_eq!(gcd::gcd(&numbers), 6);
///
/// let empty: Vec<u32> = vec![];
/// assert_eq!(gcd::gcd(&empty), 0);
/// ```
pub fn gcd(numbers: &[Pint]) -> Pint {
    if numbers.is_empty() {
        return Pint::zero();
    }
    if numbers.len() == 1 {
        return numbers[0];
    }
    let mut result = numbers[0];
    for &number in numbers.iter().skip(1) {
        result = gcd_of_two(result, number);
        if result == Pint::one() {
            return Pint::one();
        }
    }
    result
}

/// Core logic of the GCD Euclidean algorithm for only two numbers
fn gcd_of_two(r0: Pint, r1: Pint) -> Pint {
    if r1 == Pint::zero() {
        r0
    } else {
        gcd_of_two(r1, r0 % r1)
    }
}

/// The extended euclidian algorithm for calculating the gcd in equation: `au + bv = gcd(a,b)`
/// #### Arguments
/// * `a` and `b` two i64 integers
/// #### Returns
/// * `u`, `v` and `gcd(a,b)`
/// #### Examples
/// ```
/// use crypto_toolbox::utils::gcd;
///
/// let (u,v,g) = gcd::extended_gcd(12,16);
/// assert_eq!(g, 4);
/// assert_eq!(12*u + 16*v, g); // Verify Bézout's identity
/// ```
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut u: i64 = 1;
    let mut g: i64 = a;
    let mut x: i64 = 0;
    let mut y: i64 = b;

    loop {
        if y == 0 {
            let v: i64 = (g - a * u) / b;
            return (u, v, g);
        }
        let t: i64 = g % y;
        let q: i64 = (g - t) / y;
        let s: i64 = u - q * x;
        u = x;
        g = y;
        x = s;
        y = t;
    }
}
/// Computes modular exponentiation (base^exponent mod modulo) using binary exponentiation algorithm.
///
/// This function uses the binary expansion of the exponent to efficiently compute
/// the result with fewer multiplications.
///
/// # Arguments
/// * `base` - The base number
/// * `exponent` - The exponent to raise the base to
/// * `modulo` - The modulus for the operation
///
/// # Returns
/// The result of base^exponent mod modulo
///
/// # Examples
/// ```
/// use crypto_toolbox::utils::arithmatic;
///
/// assert_eq!(arithmatic::mod_pow(7, 13, 17), 2);
/// assert_eq!(arithmatic::mod_pow(4, 13, 497), 445);
/// ```
pub fn mod_pow(number: u64, exponent: u64, modulo: u64) -> u64 {
    let binary_expansion = get_binary_digits(exponent);
    println!("binary_expansion is : {binary_expansion:?}");

    let mut inter_numbers: Vec<u64> = Vec::new();
    inter_numbers.push(number % modulo);
    for i in 1..binary_expansion.len() {
        inter_numbers.push((inter_numbers[i - 1].pow(2)) % modulo);
    }
    println!("inter_numebrs is : {inter_numbers:?}");

    let mut result: u64 = 1;
    for i in 0..binary_expansion.len() {
        result *= inter_numbers[i].pow(binary_expansion[i] as u32);
        result = result % modulo
    }
    result
}

/// Converts a number to its binary representation as a vector of digits.
///
/// The resulting vector contains the binary digits in reverse order
/// (least significant bit first).
///
/// # Arguments
/// * `number` - The number to convert to binary
///
/// # Returns
/// A vector of binary digits (0s and 1s) in reverse order
///
/// # Examples
/// ```
/// use crypto_toolbox::utils::arithmatic;
///
/// assert_eq!(arithmatic::get_binary_digits(13), vec![1, 0, 1, 1]);
/// assert_eq!(arithmatic::get_binary_digits(8), vec![0, 0, 0, 1]);
/// ```
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
/// Calculates the modular multiplicative inverse using Fermat's Little Theorem.
///
/// This method only works when the modulus is a prime number.
///
/// # Arguments
/// * `base` - The number to find the inverse for
/// * `modulo` - The modulus (must be prime)
///
/// # Returns
/// * `Some(inverse)` - The modular multiplicative inverse if it exists
/// * `None` - If no inverse exists or if modulo is not prime
///
/// # Examples
/// ```
/// use crypto_toolbox::utils::arithmatic;
///
/// // Inverse of 3 modulo 11 (a prime) is 4 because (3 * 4) % 11 = 1
/// assert_eq!(arithmatic::mod_inv_fermat(3, 11), Some(4));
///
/// // No inverse exists for 2 modulo 4 (not coprime)
/// assert_eq!(arithmatic::mod_inv_fermat(2, 4), None);
/// ```
pub fn mod_inv_fermat(base: u64, modulo: u64) -> Option<u64> {
    if modulo % base == 0 {
        return None;
    }
    if mod_pow(base, modulo - 1, modulo) == 1 {
        return Some(mod_pow(base, modulo - 2, modulo));
    } else {
        return None;
    }
}
/// Calculates the modular multiplicative inverse using the Extended Euclidean Algorithm.
///
/// This method works for any modulus (prime or composite) as long as
/// the base and modulus are coprime (their GCD is 1).
///
/// # Arguments
/// * `base` - The number to find the inverse for
/// * `modulo` - The modulus
///
/// # Returns
/// * `Some(inverse)` - The modular multiplicative inverse if it exists
/// * `None` - If no inverse exists (base and modulo are not coprime)
///
/// # Examples
/// ```
/// use crypto_toolbox::utils::arithmatic;
///
/// // Inverse of 3 modulo 11 is 4 because (3 * 4) % 11 = 1
/// assert_eq!(arithmatic::mod_inv(3, 11), Some(4));
///
/// // No inverse exists for 2 modulo 4 (GCD of 2 and 4 is 2, not 1)
/// assert_eq!(arithmatic::mod_inv(2, 4), None);
/// ```
pub fn mod_inv(base: i64, modulo: i64) -> Option<i64> {
    let (inv, _, gcd) = extended_gcd(base, modulo);
    if gcd == 1 {
        Some((inv % modulo + modulo) % modulo)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crypto_bigint::rand_core::OsRng;
    use crypto_bigint::{self, Limb, NonZero, Random, U1024, U2048, Uint};
    use rand::prelude::*;

    #[test]
    fn test_crypto_bigint() {
        // let n = U2048::random(&mut OsRng);
        let n = U2048::from_u32(3);

        // let p = NonZero::new(U2048::random(&mut OsRng)).unwrap();
        let p = NonZero::new(U2048::from_u32(7)).unwrap();

        // let m = n.mul_mod(&U2048::random(&mut OsRng), &p);
        let m = n.mul_mod(&U2048::from_u32(3), &p);

        println!("n is {n:?} \n and p is {p:?} \n and m is {m:?}");
    }

    #[test]
    fn test_gcd() {
        let mut numbers: Vec<Pint> = Vec::new();
        numbers.push(Pint::from_u8(8));
        numbers.push(Pint::from_u8(4));

        let g = gcd(&numbers);
        dbg!(g);
        dbg!(numbers);
    }

    #[test]
    fn test_extended_gcd() {
        let mut rng = rand::rng();

        let a: i64 = rng.random_range(1..1000);
        let b: i64 = rng.random_range(1..1000);

        let (u, v, g) = extended_gcd(a, b);

        assert_eq!(a * u + b * v, g);

        println!("a={}, b={}, u={}, v={}, g={}", a, b, u, v, g);
        println!("Verification: {} * {} + {} * {} = {}", a, u, b, v, g);
    }
    #[test]
    fn test_mod_pow() {
        // Test basic cases
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(7, 16, 17), 1);

        // Test with very large exponent
        assert_eq!(mod_pow(2, 100, 1000), 376);

        // Test edge cases
        assert_eq!(mod_pow(5, 0, 7), 1); // Any number^0 is 1
        assert_eq!(mod_pow(0, 5, 7), 0); // 0^anything is 0
        assert_eq!(mod_pow(5, 5, 1), 0); // Anything mod 1 is 0
    }

    #[test]
    fn test_get_binary_digits() {
        // Test various numbers
        assert_eq!(get_binary_digits(0), vec![0]);
        assert_eq!(get_binary_digits(1), vec![1]);
        assert_eq!(get_binary_digits(2), vec![0, 1]);
        assert_eq!(get_binary_digits(13), vec![1, 0, 1, 1]);
        assert_eq!(get_binary_digits(255), vec![1, 1, 1, 1, 1, 1, 1, 1]);

        // Test powers of 2
        assert_eq!(get_binary_digits(8), vec![0, 0, 0, 1]);
        assert_eq!(get_binary_digits(16), vec![0, 0, 0, 0, 1]);
        assert_eq!(get_binary_digits(32), vec![0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn test_mod_inv_fermat() {
        // Test with prime moduli
        assert_eq!(mod_inv_fermat(3, 11), Some(4));
        assert_eq!(mod_inv_fermat(7, 23), Some(10));

        // Test with non-coprime values
        assert_eq!(mod_inv_fermat(2, 4), None);
        assert_eq!(mod_inv_fermat(4, 8), None);

        // Test with edge cases
        assert_eq!(mod_inv_fermat(1, 7), Some(1)); // 1 is its own inverse
    }

    #[test]
    fn test_mod_inv() {
        // Test with prime moduli
        assert_eq!(mod_inv(3, 11), Some(4));
        assert_eq!(mod_inv(7, 23), Some(10));

        // Test with composite moduli (extended Euclidean works with these)
        assert_eq!(mod_inv(5, 21), Some(17));

        // Test with non-coprime values
        assert_eq!(mod_inv(2, 4), None);
        assert_eq!(mod_inv(4, 8), None);

        // Test with negative numbers
        assert_eq!(mod_inv(-3, 11), Some(7));

        // Test with edge cases
        assert_eq!(mod_inv(1, 7), Some(1)); // 1 is its own inverse
    }

    #[test]
    fn test_inverse_verification() {
        // Verify that base * inverse ≡ 1 (mod modulo)
        let base = 5;
        let modulo = 11;

        if let Some(inv) = mod_inv(base, modulo) {
            let result = (base * inv) % modulo;
            assert_eq!(
                result, 1,
                "Inverse verification failed: {} * {} ≢ 1 (mod {})",
                base, inv, modulo
            );
        }

        if let Some(inv) = mod_inv_fermat(base as u64, modulo as u64) {
            let result = (base as u64 * inv) % (modulo as u64);
            assert_eq!(
                result, 1,
                "Fermat inverse verification failed: {} * {} ≢ 1 (mod {})",
                base, inv, modulo
            );
        }
    }
}
