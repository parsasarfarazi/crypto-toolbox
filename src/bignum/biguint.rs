use super::{error::BigUintParseError, utils::*};
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::{Binary, Debug, Display, LowerHex, UpperHex},
    ops::{
        Add, AddAssign, BitAnd, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign,
        Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    str::FromStr,
};

/// # Big Unsigned Integer Type
/// Implementation of 512, 1024, 2048 and 4096 bit unsigned integers and their corresponding operations, formatters and converters.
/// ```
/// use crypto_toolbox::bignum::U2048;
///
/// // You can create one from any of the u{8,16,32,64,128} integers.
/// let a = U2048::from(6u8);
/// let b = U2048::from(3u32);
/// let c: U2048 = 2u64.into();
///
/// // You can create it from strings too, written in 0x, 0b and decimal form.
/// let d = U2048::from_str("0x4");
///
/// // You can do the simple mathematical ops and many more! (%,&,|,^,!,>>,<<,<,>)
/// let add_result = (a + b) * c / d;
///
/// // You can convert it back to rust common integers.
/// let in_u128:u128 = add_result.into();
///
/// // And also string formatting is also provided.
/// assert_eq![add_result, U2048::from(18u8)];
/// ```
/// This type is Created using an array of u64 integers with the length of N, where `N = bigUint_bits / 64`.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BigUint<const N: usize>(pub [u64; N]);

/// 512 BigUint with 8 Limbs
pub type U512 = BigUint<8>;
/// 1024 BigUint with 16 Limbs
pub type U1024 = BigUint<16>;
/// 2048 BigUint with 32 Limbs
pub type U2048 = BigUint<32>;
/// 4096 BigUint with 64 Limbs
pub type U4096 = BigUint<64>;

impl<const N: usize> BigUint<N> {
    /// Creates a new Zero instance! equal to defualt().
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if it is Zero.
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }

    /// Count the number of Limbs used.
    pub fn digit_len(&self) -> usize {
        for i in (0..N).rev() {
            if self.0[i] != 0 {
                return i + 1;
            }
        }
        0
    }

    /// Checked Multipication with u64
    pub fn checked_mul_small(self, rhs: u64) -> Option<Self> {
        let mut res = [0u64; N];
        let mut carry: u128 = 0;

        for i in 0..N {
            let prod = (self.0[i] as u128) * (rhs as u128) + carry;
            res[i] = prod as u64; // low 64 bits
            carry = prod >> 64; // high 64 bits → next iteration
        }

        // if carry is still non-zero we overflowed 2048 bits
        if carry != 0 {
            return None;
        }
        Some(BigUint::<N>(res))
    }

    /// Checked Addition with u64
    pub fn checked_add_small(self, rhs: u64) -> Option<Self> {
        let mut res = self.0;
        let mut carry = rhs as u128;

        for i in 0..N {
            let sum = res[i] as u128 + carry;
            res[i] = sum as u64;
            carry = sum >> 64;
            if carry == 0 {
                // no more carry to propagate
                return Some(BigUint::<N>(res));
            }
        }

        // carry escaped the top limb → overflow
        if carry != 0 {
            None
        } else {
            Some(BigUint::<N>(res))
        }
    }
}

impl<const N: usize> Default for BigUint<N> {
    fn default() -> Self {
        BigUint::<N>([0; N])
    }
}

// Math Ops
impl<const N: usize> Add for BigUint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self::new();
        let mut carry = false;
        for i in 0..N {
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

impl<const N: usize> AddAssign for BigUint<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const N: usize> Sub for BigUint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Self::new();
        let mut bor = false;

        for i in 0..N {
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

impl<const N: usize> SubAssign for BigUint<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<const N: usize> Mul for BigUint<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = [0; N];

        for i in 1..N {
            let mut carry: u128 = 0;

            for j in 1..(N - i) {
                let k = j + i;

                let a = self.0[i] as u128;
                let b = rhs.0[j] as u128;

                let existing = res[k] as u128;

                let tmp = a * b + existing + carry;

                res[k] = tmp as u64;
                carry = tmp >> 64;
            }
        }

        BigUint::<N>(res)
    }
}

impl<const N: usize> MulAssign for BigUint<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const N: usize> Div for BigUint<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        div_rem(self, rhs).0
    }
}

impl<const N: usize> DivAssign for BigUint<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const N: usize> Rem for BigUint<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        div_rem(self, rhs).1
    }
}

impl<const N: usize> RemAssign for BigUint<N> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl<const N: usize> Shl<usize> for BigUint<N> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut res = [0u64; N];

        if rhs >= N {
            // shifting out all bits
            return BigUint::<N>(res);
        }

        let word_shift = rhs / 64; // how many whole u64 limbs to jump
        let bit_shift = rhs % 64; // remaining bit shift inside a limb

        if bit_shift == 0 {
            // --- aligned shift: just copy words ---
            for i in word_shift..N {
                res[i] = self.0[i - word_shift];
            }
        } else {
            for i in word_shift..N {
                // take the bits from the source limb and shift them up
                res[i] = self.0[i - word_shift] << bit_shift;

                // OR in the carry bits spilling from the limb below
                if i > word_shift {
                    res[i] |= self.0[i - word_shift - 1] >> (64 - bit_shift);
                }
            }
        }

        BigUint::<N>(res)
    }
}

impl<const N: usize> ShlAssign<usize> for BigUint<N> {
    fn shl_assign(&mut self, rhs: usize) {
        *self = *self << rhs;
    }
}

impl<const N: usize> Shr<usize> for BigUint<N> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut res = [0u64; N];

        if rhs >= N {
            return BigUint::<N>(res);
        }

        let word_shift = rhs / 64;
        let bit_shift = rhs % 64;

        if bit_shift == 0 {
            // --- aligned shift: just copy words ---
            for i in 0..(N - word_shift) {
                res[i] = self.0[i + word_shift];
            }
        } else {
            for i in 0..(N - word_shift) {
                // take bits from the source limb and shift them down
                res[i] = self.0[i + word_shift] >> bit_shift;

                // OR in the carry bits spilling from the limb above
                if i + word_shift + 1 < N {
                    res[i] |= self.0[i + word_shift + 1] << (64 - bit_shift);
                }
            }
        }

        BigUint::<N>(res)
    }
}

impl<const N: usize> ShrAssign<usize> for BigUint<N> {
    fn shr_assign(&mut self, rhs: usize) {
        *self = *self >> rhs;
    }
}

impl<const N: usize> BitAnd for BigUint<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut res = [0; N];
        for i in 0..N {
            res[i] = self.0[i] & rhs.0[i];
        }

        BigUint::<N>(res)
    }
}

impl<const N: usize> BitOr for BigUint<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut res = [0; N];
        for i in 0..N {
            res[i] = self.0[i] | rhs.0[i];
        }
        BigUint::<N>(res)
    }
}

impl<const N: usize> BitXor for BigUint<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut res = [0; N];
        for i in 0..N {
            res[i] = self.0[i] ^ rhs.0[i];
        }

        BigUint::<N>(res)
    }
}

impl<const N: usize> Not for BigUint<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut res = [0; N];
        for i in 0..N {
            res[i] = !self.0[i];
        }

        BigUint::<N>(res)
    }
}

// Order traits
impl<const N: usize> Ord for BigUint<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..N).rev() {
            match self.0[i].cmp(&other.0[i]) {
                Ordering::Equal => continue,
                ord => return ord,
            }
        }
        Ordering::Equal
    }
}

impl<const N: usize> PartialOrd for BigUint<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Format Traits
impl<const N: usize> Display for BigUint<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_decimal_string(self))
    }
}

impl<const N: usize> Debug for BigUint<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nArray of 64-bits: [")?;

        for i in 0..N {
            write!(f, "{},", self.0[i])?;
        }
        write!(f, "]\n")?;

        write!(f, "Decimal: {}\n", self)?;

        write!(f, "Hex: {:#X}\n", self)?;

        write!(f, "Bin: {:#b}\n", self)
    }
}

impl<const N: usize> LowerHex for BigUint<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        } // handles the # flag
        fmt_hex(self, f, false)
    }
}

impl<const N: usize> UpperHex for BigUint<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        fmt_hex(self, f, true)
    }
}

impl<const N: usize> Binary for BigUint<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "0b")?;
        }
        fmt_binary(self, f)
    }
}

impl<const N: usize> FromStr for BigUint<N> {
    type Err = BigUintParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ── 1. reject empty input ─────────────────────────────────────────────
        if s.is_empty() {
            return Err(BigUintParseError::Empty);
        }

        // ── 2. detect optional sign (we only support non-negative) ────────────
        //       strip a leading '+' but reject '-'
        let s = s.trim();
        let s = s.strip_prefix('+').unwrap_or(s);

        if s.is_empty() {
            return Err(BigUintParseError::Empty);
        }

        // ── 3. detect base prefix ─────────────────────────────────────────────
        let (base, digits): (u32, &str) =
            if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
                (16, hex)
            } else if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
                (2, bin)
            } else if s.starts_with("0o") || s.starts_with("0O") {
                // octal prefix: not supported, give a clear error
                return Err(BigUintParseError::InvalidPrefix(s[..2].to_string()));
            } else {
                (10, s)
            };

        if digits.is_empty() {
            return Err(BigUintParseError::Empty);
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

// Convert Traits
impl<const N: usize> From<u8> for BigUint<N> {
    fn from(value: u8) -> Self {
        let mut tmp = [0; N];
        tmp[0] = value as u64;
        BigUint::<N>(tmp)
    }
}

impl<const N: usize> From<u16> for BigUint<N> {
    fn from(value: u16) -> Self {
        let mut tmp = [0; N];
        tmp[0] = value as u64;
        BigUint::<N>(tmp)
    }
}

impl<const N: usize> From<u32> for BigUint<N> {
    fn from(value: u32) -> Self {
        let mut tmp = [0; N];
        tmp[0] = value as u64;
        BigUint::<N>(tmp)
    }
}

impl<const N: usize> From<u64> for BigUint<N> {
    fn from(value: u64) -> Self {
        let mut tmp = [0; N];
        tmp[0] = value;
        BigUint::<N>(tmp)
    }
}

impl<const N: usize> From<u128> for BigUint<N> {
    fn from(value: u128) -> Self {
        let mut tmp = [0; N];
        tmp[0] = value as u64;
        tmp[1] = (value >> 64) as u64;
        BigUint::<N>(tmp)
    }
}

impl<const N: usize> From<BigUint<N>> for u8 {
    fn from(value: BigUint<N>) -> Self {
        let tmp = value % BigUint::<N>::from(8u64);
        tmp.0[0] as u8
    }
}

impl<const N: usize> From<BigUint<N>> for u16 {
    fn from(value: BigUint<N>) -> Self {
        let tmp = value % BigUint::<N>::from(16u64);
        tmp.0[0] as u16
    }
}

impl<const N: usize> From<BigUint<N>> for u32 {
    fn from(value: BigUint<N>) -> Self {
        let tmp = value % BigUint::<N>::from(32u64);
        tmp.0[0] as u32
    }
}

impl<const N: usize> From<BigUint<N>> for u64 {
    fn from(value: BigUint<N>) -> Self {
        let tmp = value % BigUint::<N>::from(64u64);
        tmp.0[0]
    }
}

impl<const N: usize> From<BigUint<N>> for u128 {
    fn from(value: BigUint<N>) -> Self {
        (value.0[0] as u128) | ((value.0[1] as u128) << 64)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
