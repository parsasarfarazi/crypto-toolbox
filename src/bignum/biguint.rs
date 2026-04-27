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
    pub const MAX: Self = Self { 0: [u64::MAX; N] };
    /// Creates a new Zero instance! equal to defualt().
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if it is Zero.
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }

    /// Count the number of Limbs used.
    pub fn limb_count(&self) -> usize {
        for i in (0..N).rev() {
            if self.0[i] != 0 {
                return i + 1;
            }
        }
        0
    }

    /// Overflowing Addition
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let mut res = [0u64; N];
        let mut carry = false;

        for i in 0..32 {
            let (sum1, c1) = self.0[i].overflowing_add(rhs.0[i]);
            let (sum2, c2) = sum1.overflowing_add(carry as u64);

            res[i] = sum2;
            carry = c1 | c2;
        }

        (BigUint::<N> { 0: res }, carry)
    }

    /// Checked Addition
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let (sum, carry) = self.overflowing_add(rhs);
        if carry { None } else { Some(sum) }
    }

    /// Overflowing Sub
    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let mut res = [0u64; N];
        let mut bor = false;

        for i in 0..N {
            let (dif1, bor1) = self.0[i].overflowing_sub(rhs.0[i]);
            let (dif2, bor2) = dif1.overflowing_sub(bor as u64);

            res[i] = dif2;
            bor = bor1 | bor2;
        }

        (BigUint { 0: res }, bor)
    }

    /// Checked Sub
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        let (dif, bor) = self.overflowing_sub(rhs);
        if bor { None } else { Some(dif) }
    }

    /// Overflowing Multipication
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let mut res = [0; N];
        let mut overflow = false;

        for i in 0..N {
            let mut carry: u128 = 0;

            for j in 0..N {
                let k = i + j;

                if k >= N {
                    let a = self.0[i] as u128;
                    let b = rhs.0[j] as u128;
                    if a * b + carry != 0 {
                        overflow = true;
                    }
                    continue;
                }

                let a = self.0[i] as u128;
                let b = rhs.0[j] as u128;
                let existing = res[k] as u128;

                let tmp = a * b + existing + carry;

                res[k] = tmp as u64;
                carry = tmp >> 64;
            }

            if carry != 0 {
                overflow = true;
            }
        }

        (BigUint::<N> { 0: res }, overflow)
    }

    /// Checked Multipication
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let (result, overflow) = self.overflowing_mul(rhs);
        if overflow { None } else { Some(result) }
    }

    /// Division - Remainder
    pub fn div_rem(self, rhs: Self) -> (Self, Self) {
        div_rem(self, rhs)
    }
}

impl<const N: usize> Default for BigUint<N> {
    fn default() -> Self {
        BigUint::<N>([0; N])
    }
}

// ADD

impl<const N: usize> Add for BigUint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.overflowing_add(rhs).0
    }
}

impl<const N: usize> Add<u64> for BigUint<N> {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        self.overflowing_add(BigUint::<N>::from(rhs)).0
    }
}

impl<const N: usize> Add<BigUint<N>> for u64 {
    type Output = BigUint<N>;
    fn add(self, rhs: BigUint<N>) -> Self::Output {
        rhs.overflowing_add(BigUint::<N>::from(self)).0
    }
}

impl<const N: usize> AddAssign for BigUint<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const N: usize> AddAssign<u64> for BigUint<N> {
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}

// SUB

impl<const N: usize> Sub for BigUint<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.overflowing_sub(rhs).0
    }
}

impl<const N: usize> Sub<u64> for BigUint<N> {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        self.overflowing_sub(BigUint::<N>::from(rhs)).0
    }
}

impl<const N: usize> Sub<BigUint<N>> for u64 {
    type Output = BigUint<N>;

    fn sub(self, rhs: BigUint<N>) -> Self::Output {
        BigUint::<N>::from(self).overflowing_sub(rhs).0
    }
}

impl<const N: usize> SubAssign for BigUint<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<const N: usize> SubAssign<u64> for BigUint<N> {
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self + rhs
    }
}

// MUL

impl<const N: usize> Mul for BigUint<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.overflowing_mul(rhs).0
    }
}

impl<const N: usize> Mul<u64> for BigUint<N> {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        self.overflowing_mul(BigUint::<N>::from(rhs)).0
    }
}

impl<const N: usize> Mul<BigUint<N>> for u64 {
    type Output = BigUint<N>;

    fn mul(self, rhs: BigUint<N>) -> Self::Output {
        BigUint::<N>::from(self).overflowing_mul(rhs).0
    }
}

impl<const N: usize> MulAssign for BigUint<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const N: usize> MulAssign<u64> for BigUint<N> {
    fn mul_assign(&mut self, rhs: u64) {
        *self = *self + rhs
    }
}

// DIV

impl<const N: usize> Div for BigUint<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        div_rem(self, rhs).0
    }
}

impl<const N: usize> Div<u64> for BigUint<N> {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        div_rem(self, BigUint::<N>::from(rhs)).0
    }
}

impl<const N: usize> Div<BigUint<N>> for u64 {
    type Output = BigUint<N>;

    fn div(self, rhs: BigUint<N>) -> Self::Output {
        div_rem(BigUint::<N>::from(self), rhs).0
    }
}

impl<const N: usize> DivAssign for BigUint<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const N: usize> DivAssign<u64> for BigUint<N> {
    fn div_assign(&mut self, rhs: u64) {
        *self = *self / rhs
    }
}

// REM

impl<const N: usize> Rem for BigUint<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        div_rem(self, rhs).1
    }
}

impl<const N: usize> Rem<u64> for BigUint<N> {
    type Output = Self;
    fn rem(self, rhs: u64) -> Self::Output {
        div_rem(self, BigUint::<N>::from(rhs)).1
    }
}

impl<const N: usize> Rem<BigUint<N>> for u64 {
    type Output = BigUint<N>;

    fn rem(self, rhs: BigUint<N>) -> Self::Output {
        div_rem(BigUint::<N>::from(self), rhs).1
    }
}

impl<const N: usize> RemAssign for BigUint<N> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl<const N: usize> RemAssign<u64> for BigUint<N> {
    fn rem_assign(&mut self, rhs: u64) {
        *self = *self / rhs
    }
}

// SHIFT

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

// BIT OPS

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

impl<const N: usize> From<i32> for BigUint<N> {
    fn from(value: i32) -> Self {
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
    use super::*;

    // tests irrelevent to conversions are done with 2048 bit(N = 32) integers.
    #[test]
    fn test_default_new() {
        let a = U2048::default();
        let b = U2048::new();

        let zero_arr = [0; 32];

        for i in 1..32 {
            if a.0[i] != zero_arr[i] && (b.0[i] == a.0[i]) {
                panic!("default() and new() are not zero in index: {i}")
            }
        }
    }

    #[test]
    fn test_is_zero() {
        let zero = U2048 { 0: [0; 32] };
        assert_eq!(zero.is_zero(), true);

        let not_zero = U2048 { 0: [1; 32] };
        assert_eq!(not_zero.is_zero(), false);

        let first_part = [0u64; 31];
        let sec_part = [1u64; 1];
        let mut combined = [0; 32];
        combined[..31].copy_from_slice(&first_part);
        combined[31..].copy_from_slice(&sec_part);
        let not_zero = U2048 { 0: combined };

        assert_eq!(not_zero.is_zero(), false)
    }

    #[test]
    fn test_limb_count() {
        let num = [
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        assert_eq!(U2048 { 0: num }.limb_count(), 8);

        let num = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        assert_eq!(U2048 { 0: num }.limb_count(), 0);

        let num = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        assert_eq!(U2048 { 0: num }.limb_count(), 1);

        let num1 = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ];
        assert_eq!(U2048 { 0: num1 }.limb_count(), 32);
    }

    #[test]
    fn test_add() {
        let a = U2048::from(u64::MAX);
        let b = a + 2;

        assert_eq!(b.0[0], 1);
        assert_eq!(b.0[1], 1);

        let mut full = U2048::MAX;
        let overflow_test = full + 1;
        full += 1;

        assert_eq!(overflow_test, U2048::from(0u64));
        assert_eq!(full, U2048::new());

        let c = U2048::from_str("0x7823743872938924ffff839").unwrap();
        let d = U2048::from_str("7823743872938924839").unwrap();

        let e = c + d;

        dbg!(c, d, e);
        assert_eq!(e, U2048::from_str("2323816408950849785974870880").unwrap());

        let a = U2048::from_str("4397498298439248398493457487348738832983874382").unwrap();
        let b =
            U2048::from_str("84398493489384935737948193182172898192819281928928493423").unwrap();
        let res = a + b;

        assert_eq!(
            res,
            U2048::from_str("84398493493782434036387441580666355680168020761912367805").unwrap()
        );
    }
    #[test]
    fn test_sub() {
        let a = U2048::from(0u64);
        let res = a - 1;

        assert_eq!(res, U2048::MAX);

        let b = U2048::from_str("453").unwrap();
        let c = U2048::from_str("453").unwrap();

        assert_eq!(b - c, U2048::new());

        let a = U2048::from_str("4397498298439248398493457487348738832983874382").unwrap();
        let b =
            U2048::from_str("84398493489384935737948193182172898192819281928928493423").unwrap();
        let res = b - a;

        assert_eq!(
            res,
            U2048::from_str("84398493484987437439508944783679440705470543095944619041").unwrap()
        );
    }

    #[test]
    fn test_mul() {
        let a = U2048::from(2_u64);
        let b = U2048::from(3_u64);
        let res = a * b;
        assert_eq!(res, U2048::from(6_u64));

        let a = U2048::from(10_u64);
        let res = a * 0;

        assert_eq!(res, U2048::from(0_u64));

        let a = U2048::from_str("4397498298439248398493457487348738832983874382").unwrap();
        let b =
            U2048::from_str("84398493489384935737948193182172898192819281928928493423").unwrap();
        let res = b * a;

        assert_eq!(
            res,
            U2048::from_str("371142231510406239074939385092297999096903664249370481200019161205345534114791825654364991324745189586").unwrap()
        );
    }

    #[test]
    fn test_div() {
        let a = U2048::from(16_u64);
        let res = a / 4;

        assert_eq!(res, U2048::from(4_u64));
        let a = U2048::from_str("4397498298439248398493457487348738832983874382").unwrap();
        let b =
            U2048::from_str("84398493489384935737948193182172898192819281928928493423").unwrap();
        let res = b / a;

        assert_eq!(res, U2048::from_str("19192387980").unwrap());
    }

    #[test]
    fn test_rem() {
        let a = U2048::from(9_u64);
        let res = a % 4;

        assert_eq!(res, U2048::from(1_u64));

        let a = U2048::from_str("4397498298439248398493457487348738832983874382").unwrap();
        let b =
            U2048::from_str("84398493489384935737948193182172898192819281928928493423").unwrap();
        let res = b % a;

        assert_eq!(
            res,
            U2048::from_str("4349052014468109593339960946500343706001765063").unwrap()
        );
    }

    #[test]
    fn test_shift() {
        let a = U2048::from_str("4").unwrap();
        let res = a << 2;

        assert_eq!(res, U2048::from_str("16").unwrap());

        let a = U2048::from_str("8").unwrap();
        let res = a >> 2;

        assert_eq!(res, U2048::from_str("2").unwrap());
    }

    #[test]
    fn test_bit_ops() {
        let a = U2048::new();
        let res = !a;

        assert_eq!(res, U2048::MAX);

        let a = U2048::from_str("0b1001").unwrap();
        let b = U2048::from_str("0b0110").unwrap();

        let res = a & b;
        assert_eq!(res, U2048::new());

        let a = U2048::from_str("0b1001").unwrap();
        let b = U2048::from_str("0b0110").unwrap();

        let res = a | b;
        assert_eq!(res, U2048::from_str("0b1111").unwrap());

        let a = U2048::from_str("0b1000").unwrap();
        let b = U2048::from_str("0b1110").unwrap();

        let res = a ^ b;
        assert_eq!(res, U2048::from_str("0b0110").unwrap());
    }
}
