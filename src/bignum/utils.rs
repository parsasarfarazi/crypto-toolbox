use super::{biguint::BigUint, error::BigUintParseError};

/// Knuth-D division algorithm.
pub fn div_rem<const N: usize>(lhs: BigUint<N>, rhs: BigUint<N>) -> (BigUint<N>, BigUint<N>) {
    assert!(!rhs.is_zero(), "division by zero");

    let u = &lhs.0;
    let v = &rhs.0;

    let n = rhs.digit_len(); // significant digits of divisor
    let m_plus_n = lhs.digit_len(); // significant digits of dividend

    // --- trivial cases ---

    // dividend < divisor → quotient = 0, remainder = dividend
    if m_plus_n < n {
        return (BigUint::<N>::new(), lhs);
    }

    let m = m_plus_n - n; // quotient will have m+1 digits

    // single-digit divisor: simple long division
    if n == 1 {
        return div_rem_single(u, m_plus_n, v[0]);
    }

    // --- D1. Normalize ---
    // Choose d (a power of 2) so the top bit of v[n-1] is set.
    let d = v[n - 1].leading_zeros();

    // Shift u left by d bits  →  un[] has m+n+1 digits
    let mut un = vec![0u64; m_plus_n + 1];
    if d > 0 {
        un[0] = u[0] << d;
        for i in 1..m_plus_n {
            un[i] = (u[i] << d) | (u[i - 1] >> (64 - d));
        }
        un[m_plus_n] = u[m_plus_n - 1] >> (64 - d);
    } else {
        un[..m_plus_n].copy_from_slice(&u[..m_plus_n]);
        un[m_plus_n] = 0;
    }

    // Shift v left by d bits  →  vn[] has n digits
    let mut vn = vec![0u64; n];
    if d > 0 {
        vn[0] = v[0] << d;
        for i in 1..n {
            vn[i] = (v[i] << d) | (v[i - 1] >> (64 - d));
        }
    } else {
        vn[..n].copy_from_slice(&v[..n]);
    }

    let mut q = [0u64; N];

    // --- D2 / D7. Main loop ---
    // j goes from m down to 0
    for j in (0..=m).rev() {
        // --- D3. Estimate q_hat ---
        // q_hat = floor((un[j+n]*B + un[j+n-1]) / vn[n-1])
        let u_hi = un[j + n];
        let u_lo = un[j + n - 1];

        let (mut q_hat, mut r_hat) = if u_hi >= vn[n - 1] {
            // would overflow: clamp to B-1
            (u64::MAX, u_hi.wrapping_sub(vn[n - 1]).wrapping_add(u_lo))
        } else {
            // safe 128-bit division
            let num = ((u_hi as u128) << 64) | (u_lo as u128);
            let qh = (num / vn[n - 1] as u128) as u64;
            let rh = (num % vn[n - 1] as u128) as u64;
            (qh, rh)
        };

        // --- D3. Refine q_hat ---
        // While q_hat*vn[n-2] > B*r_hat + un[j+n-2], decrease q_hat by 1.
        // We do at most 2 iterations (proven by Knuth).
        loop {
            let lhs = (q_hat as u128) * (vn[n - 2] as u128);
            let rhs_val = ((r_hat as u128) << 64) | (un[j + n - 2] as u128);
            if lhs <= rhs_val {
                break;
            }
            q_hat -= 1;
            let (new_r, overflow) = r_hat.overflowing_add(vn[n - 1]);
            r_hat = new_r;
            if overflow {
                break; // r_hat overflowed 64 bits → condition can't hold
            }
        }

        // --- D4. Multiply and subtract ---
        // un[j..j+n] -= q_hat * vn[0..N]
        let mut borrow: i128 = 0;
        for i in 0..N {
            let prod = (q_hat as u128) * (vn[i] as u128);
            // subtract prod + borrow from un[j+i]
            let val = (un[j + i] as i128) - (prod as i128 & 0xFFFF_FFFF_FFFF_FFFF) - borrow;
            un[j + i] = val as u64; // low 64 bits
            borrow = (prod >> 64) as i128 - (val >> 64);
        }
        let val = (un[j + n] as i128) - borrow;
        un[j + n] = val as u64;

        // --- D5. Store q digit ---
        q[j] = q_hat;

        // --- D6. Add back (if we subtracted too much) ---
        if val < 0 {
            q[j] -= 1;
            let mut carry: u64 = 0;
            for i in 0..N {
                let (s1, c1) = un[j + i].overflowing_add(vn[i]);
                let (s2, c2) = s1.overflowing_add(carry);
                un[j + i] = s2;
                carry = (c1 as u64) + (c2 as u64);
            }
            un[j + n] = un[j + n].wrapping_add(carry);
        }
    }

    // --- D8. Unnormalize remainder ---
    // The remainder lives in un[0..N], shift right by d bits.
    let mut rem = [0u64; N];
    if d > 0 {
        for i in 0..N - 1 {
            rem[i] = (un[i] >> d) | (un[i + 1] << (64 - d));
        }
        rem[n - 1] = un[n - 1] >> d;
    } else {
        rem[..n].copy_from_slice(&un[..n]);
    }

    (BigUint::<N>(q), BigUint::<N>(rem))
}

fn div_rem_single<const N: usize>(u: &[u64; N], len: usize, v: u64) -> (BigUint<N>, BigUint<N>) {
    let mut q = [0u64; N];
    let mut rem: u64 = 0;
    for i in (0..len).rev() {
        let cur = ((rem as u128) << 64) | (u[i] as u128);
        q[i] = (cur / v as u128) as u64;
        rem = (cur % v as u128) as u64;
    }
    let mut r = [0u64; N];
    r[0] = rem;
    (BigUint::<N>(q), BigUint::<N>(r))
}

pub fn to_decimal_string<const N: usize>(input: &BigUint<N>) -> String {
    if input.is_zero() {
        return "0".to_string();
    }

    let ten = {
        let mut t = [0u64; N];
        t[0] = 10;
        BigUint::<N>(t)
    };

    let mut digits = Vec::new(); // digits in reverse order
    let mut cur = *input;

    while !cur.is_zero() {
        let (q, r) = div_rem(cur, ten);
        digits.push(b'0' + r.0[0] as u8); // r is always 0..=9
        cur = q;
    }

    digits.reverse();
    String::from_utf8(digits).unwrap() // safe: only ASCII digits
}

pub fn fmt_hex<const N: usize>(
    val: &BigUint<N>,
    f: &mut std::fmt::Formatter<'_>,
    upper: bool,
) -> std::fmt::Result {
    let top = (0..N).rev().find(|&i| val.0[i] != 0);

    let Some(top) = top else {
        return write!(f, "0"); // value is zero
    };

    for i in (0..=top).rev() {
        if i == top {
            // first limb: no leading zeros
            if upper {
                write!(f, "{:X}", val.0[i])?;
            } else {
                write!(f, "{:x}", val.0[i])?;
            }
        } else {
            // remaining limbs: pad to 16 hex digits
            if upper {
                write!(f, "{:016X}", val.0[i])?;
            } else {
                write!(f, "{:016x}", val.0[i])?;
            }
        }
    }
    Ok(())
}

pub fn fmt_binary<const N: usize>(
    val: &BigUint<N>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    let top = (0..N).rev().find(|&i| val.0[i] != 0);

    let Some(top) = top else {
        return write!(f, "0");
    };

    for i in (0..=top).rev() {
        if i == top {
            write!(f, "{:b}", val.0[i])?; // no leading zeros on first limb
        } else {
            write!(f, "{:064b}", val.0[i])?; // pad the rest to 64 bits
        }
    }
    Ok(())
}

pub fn parse_binary<const N: usize>(s: &str) -> Result<BigUint<N>, BigUintParseError> {
    let mut acc = BigUint::<N>::new();

    for ch in s.chars() {
        let bit = match ch {
            '0' => 0u64,
            '1' => 1u64,
            '_' => continue, // allow visual separators: 1010_0011
            _ => return Err(BigUintParseError::InvalidDigit { char: ch, base: 2 }),
        };

        // Check if top bit would be pushed out before shifting
        // top bit of acc is bit 2047, i.e. limb 31 bit 63
        if acc.0[31] >> 63 != 0 {
            return Err(BigUintParseError::Overflow);
        }

        acc = acc << 1;
        acc.0[0] |= bit;
    }

    Ok(acc)
}

pub fn parse_hex<const N: usize>(s: &str) -> Result<BigUint<N>, BigUintParseError> {
    let mut acc = BigUint::<N>::new();

    for ch in s.chars() {
        let nibble = match ch {
            '0'..='9' => (ch as u64) - ('0' as u64),
            'a'..='f' => (ch as u64) - ('a' as u64) + 10,
            'A'..='F' => (ch as u64) - ('A' as u64) + 10,
            '_' => continue, // allow: 0xDEAD_BEEF
            _ => return Err(BigUintParseError::InvalidDigit { char: ch, base: 16 }),
        };

        // check top 4 bits before shifting
        if acc.0[31] >> 60 != 0 {
            return Err(BigUintParseError::Overflow);
        }

        acc = acc << 4;
        acc.0[0] |= nibble;
    }

    Ok(acc)
}

pub fn parse_decimal<const N: usize>(s: &str) -> Result<BigUint<N>, BigUintParseError> {
    let mut acc = BigUint::<N>::new();

    for ch in s.chars() {
        let digit = match ch {
            '0'..='9' => (ch as u64) - ('0' as u64),
            '_' => continue, // allow: 1_000_000
            _ => return Err(BigUintParseError::InvalidDigit { char: ch, base: 10 }),
        };

        acc = acc
            .checked_mul_small(10)
            .ok_or(BigUintParseError::Overflow)?;

        acc = acc
            .checked_add_small(digit)
            .ok_or(BigUintParseError::Overflow)?;
    }

    Ok(acc)
}
