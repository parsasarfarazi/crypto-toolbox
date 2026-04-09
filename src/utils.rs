/// ## Greatest Common Divisor
/// implements the simple euclidian algorithm to calculate the gcd of two numbers `a` and `b`.
/// ```
///     use crypto_toolbox::utils;  
///     let g = utils::gcd(12,54);
///     assert_eq!(g,6);
/// ```
pub fn gcd(a: u128, b: u128) -> u128 {
    if a == b {
        return a;
    } else if a > b {
        if b == 0 { a } else { gcd_core(a, b) }
    } else {
        if a == 0 { b } else { gcd_core(b, a) }
    }
}

fn gcd_core(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    } else {
        gcd_core(b, a % b)
    }
}

/// ## GCD with the multiples of inputs
/// implements the extended euclidian algorithm to output `(u,v,g)` in equality au + bv = (g)cd.
/// ```
///     use crypto_toolbox::utils;  
///     let (u,v,g) = utils::extended_gcd(24,20);
///     assert_eq!(g,4);
///     assert_eq!(g as i128 ,24*u + 20*v)
/// ```
pub fn extended_gcd(a: u128, b: u128) -> (i128, i128, u128) {
    if b == 0 {
        return (1_i128, 0_i128, a);
    }
    if a == 0 {
        return (0_i128, 1_i128, b);
    }
    let a = a as i128;
    let b = b as i128;

    let mut g = a;
    let mut y = b;
    let mut u = 1;
    let mut x = 0;

    loop {
        if y == 0 {
            let v = (g - a * u) / b;
            return (u, v, g as u128);
        }
        let t = g % y;
        let q = (g - t) / y;
        let s = u - q * x;
        u = x;
        g = y;
        x = s;
        y = t;
    }
}

/// ## Modular Powering
/// uses Fast powering algorithm to compute the g^a (mod n).
/// ```
///     use crypto_toolbox::utils;  
///     let result = utils::mod_pow(100,2,7);
///     assert_eq!(result,28);
/// ```
pub fn mod_pow(n: u128, g: u128, a: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut a = a;
    let mut x = g;
    let mut b = 1;
    while a > 0 {
        if a % 2 == 1 {
            b = (b * x) % n;
        }
        x = x.pow(2) % n;
        a = a / 2;
    }
    b
}


/// ## Modular Inverse
/// calculates the modular inverse of number
/// ```
///     use crypto_toolbox::utils;  
///     let mut result = utils::mod_inv(7,5);
///     assert_eq!(result,3);
/// ```
pub fn mod_inv(n: u128, g: u128) -> u128 {
    if gcd(n, g) == 1 {
        mod_pow(n, g, n - 2)
    } else {
        0
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        let mut g = gcd(24, 12);
        assert_eq!(g, 12);

        g = gcd(23, 17);
        assert_eq!(g, 1);

        g = gcd(24, 0);
        assert_eq!(g, 24);

        g = gcd(0, 0);
        assert_eq!(g, 0)
    }

    #[test]
    fn test_extended_gcd() {
        let (mut u, mut v, mut g) = extended_gcd(24, 20);
        assert_eq!(g as i128, 24 * u + 20 * v);

        (u, v, g) = extended_gcd(23, 43);
        assert_eq!(g as i128, 23 * u + 43 * v);

        (u, v, g) = extended_gcd(0, 0);
        assert_eq!(g as i128, 0 * u + 0 * v);

        (u, v, g) = extended_gcd(23, 23);
        assert_eq!(g as i128, 23 * u + 23 * v);

        (u, v, g) = extended_gcd(12, 0);
        assert_eq!(g as i128, 12 * u + 0 * v);

        (u, v, g) = extended_gcd(0, 9);
        assert_eq!(g as i128, 0 * u + 9 * v);

        (u, v, g) = extended_gcd(4, 26);
        assert_eq!(g as i128, 4 * u + 26 * v);
    }

    #[test]
    fn test_mod_pow() {
        let mut res = mod_pow(1000, 3, 218);
        assert_eq!(res, 489);

        res = mod_pow(10, 2, 4);
        assert_eq!(res, 6);

        res = mod_pow(10, 7, 0);
        assert_eq!(res, 1);

        res = mod_pow(10, 0, 3);
        assert_eq!(res, 0);
    }


    #[test]
    fn test_mod_inv() {
        let result = mod_inv(7, 5);
        assert_eq!(result, 3);
    }
}
