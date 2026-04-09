use std::{error::Error, fmt::Display};

/// # Parse Errors for BigUint
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BigUintParseError {
    /// string is empty, "0x" and "+0b" etc. are included.
    Empty,
    /// digits doesn't match with the specified base provided, e.g. 5 in binary format.
    InvalidDigit { char: char, base: u32 },
    /// input Number is greater than the type.
    Overflow,
    /// anything other than {'0b','0x','0X'} is not implemented.(decimal doesn't need prefix)
    InvalidPrefix(String),
}

impl Display for BigUintParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BigUintParseError::Empty => write!(f, "Can't Parse from an Empty input"),
            BigUintParseError::InvalidDigit { char, base } => {
                write!(f, "Invalid Digit '{char}' for base '{base}")
            }
            BigUintParseError::Overflow => write!(f, "Value too Large to fit in BigUint<N>"),
            BigUintParseError::InvalidPrefix(p) => {
                write!(f, "Prefix '{p}' Not Recognized, Only 0x, 0b or decimal!")
            }
        }
    }
}
impl Error for BigUintParseError {}
