// From / TryFrom (to convert from u64, u128, byte arrays.
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    error::Error,
    fmt::{Binary, Debug, Display, LowerHex, UpperHex},
    ops::{
        Add, AddAssign, BitAnd, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign,
        Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    str::FromStr,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ParseU2048Error {
    Empty,
    InvalidDigit { char: char, base: u32 },
    Overflow,
    InvalidPrefix(String),
}

impl Display for ParseU2048Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseU2048Error::Empty => write!(f, "Can't Parse from an Empty input"),
            ParseU2048Error::InvalidDigit { char, base } => {
                write!(f, "Invalid Digit '{char}' for base '{base}")
            }
            ParseU2048Error::Overflow => write!(f, "Value too Large to fit in U2048"),
            ParseU2048Error::InvalidPrefix(p) => {
                write!(f, "Prefix '{p}' Not Recognized, Only 0x, 0b or decimal!")
            }
        }
    }
}

impl Error for ParseU2048Error {}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct U2048([u64; 32]);

impl Display for U2048 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_decimal_string())
    }
}

impl Debug for U2048 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nArray of 64-bits: [")?;

        for i in 0..32 {
            write!(f, "{},", self.0[i])?;
        }
        write!(f, "]\n")?;

        write!(f, "Decimal: {}\n", self)?;

        write!(f, "Hex: {:#X}\n", self)?;

        write!(f, "Bin: {:#b}\n", self)
    }
}

impl LowerHex for U2048 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        } // handles the # flag
        fmt_hex(self, f, false)
    }
}

impl UpperHex for U2048 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        fmt_hex(self, f, true)
    }
}

impl Binary for U2048 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0b")?;
        }
        fmt_binary(self, f)
    }
}

fn fmt_hex(val: &U2048, f: &mut std::fmt::Formatter<'_>, upper: bool) -> std::fmt::Result {
    let top = (0..32).rev().find(|&i| val.0[i] != 0);

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

fn fmt_binary(val: &U2048, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let top = (0..32).rev().find(|&i| val.0[i] != 0);

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

impl BitAnd for U2048 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut res = [0; 32];
        for i in 0..32 {
            res[i] = self.0[i] & rhs.0[i];
        }

        U2048(res)
    }
}

impl BitOr for U2048 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut res = [0; 32];
        for i in 0..32 {
            res[i] = self.0[i] | rhs.0[i];
        }
        U2048(res)
    }
}

impl Not for U2048 {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut res = [0; 32];
        for i in 0..32 {
            res[i] = !self.0[i];
        }

        U2048(res)
    }
}

impl BitXor for U2048 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut res = [0; 32];
        for i in 0..32 {
            res[i] = self.0[i] ^ rhs.0[i];
        }

        U2048(res)
    }
}

impl Shl<usize> for U2048 {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut res = [0u64; 32];

        if rhs >= 2048 {
            // shifting out all bits
            return U2048(res);
        }

        let word_shift = rhs / 64; // how many whole u64 limbs to jump
        let bit_shift = rhs % 64; // remaining bit shift inside a limb

        if bit_shift == 0 {
            // --- aligned shift: just copy words ---
            for i in word_shift..32 {
                res[i] = self.0[i - word_shift];
            }
        } else {
            for i in word_shift..32 {
                // take the bits from the source limb and shift them up
                res[i] = self.0[i - word_shift] << bit_shift;

                // OR in the carry bits spilling from the limb below
                if i > word_shift {
                    res[i] |= self.0[i - word_shift - 1] >> (64 - bit_shift);
                }
            }
        }

        U2048(res)
    }
}

impl Shr<usize> for U2048 {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut res = [0u64; 32];

        if rhs >= 2048 {
            return U2048(res);
        }

        let word_shift = rhs / 64;
        let bit_shift = rhs % 64;

        if bit_shift == 0 {
            // --- aligned shift: just copy words ---
            for i in 0..(32 - word_shift) {
                res[i] = self.0[i + word_shift];
            }
        } else {
            for i in 0..(32 - word_shift) {
                // take bits from the source limb and shift them down
                res[i] = self.0[i + word_shift] >> bit_shift;

                // OR in the carry bits spilling from the limb above
                if i + word_shift + 1 < 32 {
                    res[i] |= self.0[i + word_shift + 1] << (64 - bit_shift);
                }
            }
        }

        U2048(res)
    }
}

impl ShlAssign<usize> for U2048 {
    fn shl_assign(&mut self, rhs: usize) {
        *self = *self << rhs;
    }
}

impl ShrAssign<usize> for U2048 {
    fn shr_assign(&mut self, rhs: usize) {
        *self = *self >> rhs;
    }
}

impl Div for U2048 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).0
    }
}

impl DivAssign for U2048 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Rem for U2048 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).1
    }
}

impl RemAssign for U2048 {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}
// incomplete
impl Mul for U2048 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = [0; 32];

        for i in 1..32 {
            let mut carry: u128 = 0;

            for j in 1..(32 - i) {
                let k = j + i;

                let a = self.0[i] as u128;
                let b = rhs.0[j] as u128;

                let existing = res[k] as u128;

                let tmp = a * b + existing + carry;

                res[k] = tmp as u64;
                carry = tmp >> 64;
            }
        }

        U2048(res)
    }
}

impl MulAssign for U2048 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
// incomplete
impl Sub for U2048 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Self::new();
        let mut bor = false;

        for i in 0..32 {
            let (dif, borr) = self.0[i].overflowing_sub(rhs.0[i]);

            if bor {
                match dif.checked_sub(1) {
                    Some(k) => res.0[i] = k,
                    None => {
                        res.0[i] = u64::MAX;
                        bor = true;
                        continue;
                    }
                }
            } else {
                res.0[i] = dif;
            }
            bor = borr
        }

        res
    }
}

impl SubAssign for U2048 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
// incomplete
impl Add for U2048 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self::new();
        let mut carry = false;
        for i in 0..32 {
            let (sum, carryy) = self.0[i].overflowing_add(rhs.0[i]);
            if carry {
                match sum.checked_add(1) {
                    Some(k) => res.0[i] = k,
                    None => {
                        res.0[i] = 0;
                        carry = true;
                        continue;
                    }
                }
            } else {
                res.0[i] = sum;
            }
            carry = carryy;
        }
        res
    }
}

impl AddAssign for U2048 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl U2048 {
    fn new() -> Self {
        Self::default()
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }

    fn digit_len(&self) -> usize {
        for i in (0..32).rev() {
            if self.0[i] != 0 {
                return i + 1;
            }
        }
        0
    }

    pub fn div_rem(self, rhs: Self) -> (Self, Self) {
        assert!(!rhs.is_zero(), "division by zero");

        let u = &self.0;
        let v = &rhs.0;

        let n = rhs.digit_len(); // significant digits of divisor
        let m_plus_n = self.digit_len(); // significant digits of dividend

        // --- trivial cases ---

        // dividend < divisor → quotient = 0, remainder = dividend
        if m_plus_n < n {
            return (U2048::new(), self);
        }

        let m = m_plus_n - n; // quotient will have m+1 digits

        // single-digit divisor: simple long division
        if n == 1 {
            return Self::div_rem_single(u, m_plus_n, v[0]);
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

        let mut q = [0u64; 32];

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
            // un[j..j+n] -= q_hat * vn[0..n]
            let mut borrow: i128 = 0;
            for i in 0..n {
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
                for i in 0..n {
                    let (s1, c1) = un[j + i].overflowing_add(vn[i]);
                    let (s2, c2) = s1.overflowing_add(carry);
                    un[j + i] = s2;
                    carry = (c1 as u64) + (c2 as u64);
                }
                un[j + n] = un[j + n].wrapping_add(carry);
            }
        }

        // --- D8. Unnormalize remainder ---
        // The remainder lives in un[0..n], shift right by d bits.
        let mut rem = [0u64; 32];
        if d > 0 {
            for i in 0..n - 1 {
                rem[i] = (un[i] >> d) | (un[i + 1] << (64 - d));
            }
            rem[n - 1] = un[n - 1] >> d;
        } else {
            rem[..n].copy_from_slice(&un[..n]);
        }

        (U2048(q), U2048(rem))
    }

    fn div_rem_single(u: &[u64; 32], len: usize, v: u64) -> (U2048, U2048) {
        let mut q = [0u64; 32];
        let mut rem: u64 = 0;
        for i in (0..len).rev() {
            let cur = ((rem as u128) << 64) | (u[i] as u128);
            q[i] = (cur / v as u128) as u64;
            rem = (cur % v as u128) as u64;
        }
        let mut r = [0u64; 32];
        r[0] = rem;
        (U2048(q), U2048(r))
    }

    fn checked_mul_small(self, rhs: u64) -> Option<Self> {
        let mut res = [0u64; 32];
        let mut carry: u128 = 0;

        for i in 0..32 {
            let prod = (self.0[i] as u128) * (rhs as u128) + carry;
            res[i] = prod as u64; // low 64 bits
            carry = prod >> 64; // high 64 bits → next iteration
        }

        // if carry is still non-zero we overflowed 2048 bits
        if carry != 0 {
            return None;
        }
        Some(U2048(res))
    }

    fn checked_add_small(self, rhs: u64) -> Option<Self> {
        let mut res = self.0;
        let mut carry = rhs as u128;

        for i in 0..32 {
            let sum = res[i] as u128 + carry;
            res[i] = sum as u64;
            carry = sum >> 64;
            if carry == 0 {
                // no more carry to propagate
                return Some(U2048(res));
            }
        }

        // carry escaped the top limb → overflow
        if carry != 0 { None } else { Some(U2048(res)) }
    }

    fn to_decimal_string(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }

        let ten = {
            let mut t = [0u64; 32];
            t[0] = 10;
            U2048(t)
        };

        let mut digits = Vec::new(); // digits in reverse order
        let mut cur = *self;

        while !cur.is_zero() {
            let (q, r) = cur.div_rem(ten);
            digits.push(b'0' + r.0[0] as u8); // r is always 0..=9
            cur = q;
        }

        digits.reverse();
        String::from_utf8(digits).unwrap() // safe: only ASCII digits
    }
}

impl Ord for U2048 {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..32).rev() {
            match self.0[i].cmp(&other.0[i]) {
                Ordering::Equal => continue,
                ord => return ord,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for U2048 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for U2048 {
    fn default() -> Self {
        U2048([0; 32])
    }
}

impl FromStr for U2048 {
    type Err = ParseU2048Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ── 1. reject empty input ─────────────────────────────────────────────
        if s.is_empty() {
            return Err(ParseU2048Error::Empty);
        }

        // ── 2. detect optional sign (we only support non-negative) ────────────
        //       strip a leading '+' but reject '-'
        let s = s.trim();
        let s = s.strip_prefix('+').unwrap_or(s);

        if s.is_empty() {
            return Err(ParseU2048Error::Empty);
        }

        // ── 3. detect base prefix ─────────────────────────────────────────────
        let (base, digits): (u32, &str) =
            if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
                (16, hex)
            } else if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
                (2, bin)
            } else if s.starts_with("0o") || s.starts_with("0O") {
                // octal prefix: not supported, give a clear error
                return Err(ParseU2048Error::InvalidPrefix(s[..2].to_string()));
            } else {
                (10, s)
            };

        if digits.is_empty() {
            return Err(ParseU2048Error::Empty);
        }

        // ── 4. dispatch to the right parser ───────────────────────────────────
        match base {
            2 => parse_binary(digits),
            10 => parse_decimal(digits),
            16 => parse_hex(digits),
            _ => unreachable!(),
        }
    }
}

fn parse_binary(s: &str) -> Result<U2048, ParseU2048Error> {
    let mut acc = U2048::new();

    for ch in s.chars() {
        let bit = match ch {
            '0' => 0u64,
            '1' => 1u64,
            '_' => continue, // allow visual separators: 1010_0011
            _ => return Err(ParseU2048Error::InvalidDigit { char: ch, base: 2 }),
        };

        // Check if top bit would be pushed out before shifting
        // top bit of acc is bit 2047, i.e. limb 31 bit 63
        if acc.0[31] >> 63 != 0 {
            return Err(ParseU2048Error::Overflow);
        }

        acc = acc << 1;
        acc.0[0] |= bit;
    }

    Ok(acc)
}

fn parse_hex(s: &str) -> Result<U2048, ParseU2048Error> {
    let mut acc = U2048::new();

    for ch in s.chars() {
        let nibble = match ch {
            '0'..='9' => (ch as u64) - ('0' as u64),
            'a'..='f' => (ch as u64) - ('a' as u64) + 10,
            'A'..='F' => (ch as u64) - ('A' as u64) + 10,
            '_' => continue, // allow: 0xDEAD_BEEF
            _ => return Err(ParseU2048Error::InvalidDigit { char: ch, base: 16 }),
        };

        // check top 4 bits before shifting
        if acc.0[31] >> 60 != 0 {
            return Err(ParseU2048Error::Overflow);
        }

        acc = acc << 4;
        acc.0[0] |= nibble;
    }

    Ok(acc)
}

fn parse_decimal(s: &str) -> Result<U2048, ParseU2048Error> {
    let mut acc = U2048::new();

    for ch in s.chars() {
        let digit = match ch {
            '0'..='9' => (ch as u64) - ('0' as u64),
            '_' => continue, // allow: 1_000_000
            _ => return Err(ParseU2048Error::InvalidDigit { char: ch, base: 10 }),
        };

        acc = acc.checked_mul_small(10).ok_or(ParseU2048Error::Overflow)?;

        acc = acc
            .checked_add_small(digit)
            .ok_or(ParseU2048Error::Overflow)?;
    }

    Ok(acc)
}

impl From<u8> for U2048 {
    fn from(value: u8) -> Self {
        let mut tmp = [0; 32];
        tmp[0] = value as u64;
        U2048(tmp)
    }
}

impl From<u16> for U2048 {
    fn from(value: u16) -> Self {
        let mut tmp = [0; 32];
        tmp[0] = value as u64;
        U2048(tmp)
    }
}

impl From<u32> for U2048 {
    fn from(value: u32) -> Self {
        let mut tmp = [0; 32];
        tmp[0] = value as u64;
        U2048(tmp)
    }
}

impl From<u64> for U2048 {
    fn from(value: u64) -> Self {
        let mut tmp = [0; 32];
        tmp[0] = value;
        U2048(tmp)
    }
}

impl From<u128> for U2048 {
    fn from(value: u128) -> Self {
        let mut tmp = [0; 32];
        tmp[0] = value as u64;
        tmp[1] = (value >> 64) as u64;
        U2048(tmp)
    }
}

impl From<U2048> for u8 {
    fn from(value: U2048) -> Self {
        let tmp = value % U2048::from(8u64);
        tmp.0[0] as u8
    }
}

impl From<U2048> for u16 {
    fn from(value: U2048) -> Self {
        let tmp = value % U2048::from(16u64);
        tmp.0[0] as u16
    }
}

impl From<U2048> for u32 {
    fn from(value: U2048) -> Self {
        let tmp = value % U2048::from(32u64);
        tmp.0[0] as u32
    }
}

impl From<U2048> for u64 {
    fn from(value: U2048) -> Self {
        let tmp = value % U2048::from(64u64);
        tmp.0[0]
    }
}

impl From<U2048> for u128 {
    fn from(value: U2048) -> Self {
        (value.0[0] as u128) | ((value.0[1] as u128) << 64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let first = U2048 {
            0: [
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value(),
                u64::max_value() - 1,
            ],
        };
        let second = U2048 {
            0: [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        };
        let result = U2048 {
            0: [
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                u64::max_value(),
            ],
        };

        let test = first + second;

        println!("test is {:} and first is {:}", test, first);
        assert_eq!(test, result)
    }

    #[test]
    fn test_sub() {
        let first = U2048 {
            0: [
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value(),
                // u64::max_value() - 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
        };
        let second = U2048 {
            0: [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        };
        let expected = U2048 {
            0: [
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                u64::MAX - 1,
                0,
            ],
        };

        let result = first - second;

        println!("result is {:?} and expected is {:?}", result, expected);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_display() {
        let num = U2048::from_str(
            "478392938434839487387548573848349853984938583849385934934857023484989348",
        )
        .unwrap();

        println!("The pretty format is like : {}", num);
        println!("The debug format is like : {:?}", num);
    }
}
