/// Calculates the greatest common divisors of a vector of numbers using the euclidian algorithm
/// #### Arguments
/// * `numbers` - A reference to a array of unsigned 32-bit integers
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
pub fn gcd(numbers: &[u32]) -> u32 {
    if numbers.is_empty() {
        return 0;
    }
    if numbers.len() == 1 {
        return numbers[0];
    }
    let mut result = numbers[0];
    for &number in numbers.iter().skip(1) {
        result = gcd_of_two(result, number);
        if result == 1 {
            return 1;
        }
    }
    result
}

/// Core logic of the GCD Euclidean algorithm for only two numbers
fn gcd_of_two(r0: u32, r1: u32) -> u32 {
    if r1 == 0 { r0 } else { gcd_of_two(r1, r0 % r1) }
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

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_gcd() {
        let  random = rand::rng();
        let numbers: Vec<u32> = random.random_iter().take(4).collect();
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
}
