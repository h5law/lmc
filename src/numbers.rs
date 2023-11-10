use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

// Flag are used to indicate the state of the last operation
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Flag {
    NEG,
    OVERFLOW,
}

// ThreeDigitNumber is a 3-digit decimal number from 000-999
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ThreeDigitNumber(i16, Option<Flag>);

impl ThreeDigitNumber {
    pub fn new(value: i16) -> Result<Self, String> {
        if 0 <= value && value <= 999 {
            Ok(ThreeDigitNumber(value, None))
        } else {
            Err("Out of bounds: must be in the range 000-999".into())
        }
    }

    pub fn new_with_flag(value: i16, flag: Option<Flag>) -> Result<Self, String> {
        if 0 <= value && value <= 999 {
            Ok(ThreeDigitNumber(value, flag))
        } else {
            Err("Out of bounds: must be in the range 000-999".into())
        }
    }

    pub fn value(&self) -> i16 {
        self.0
    }

    pub fn flag(&self) -> Option<Flag> {
        self.1
    }
}

// Implement the Add trait for ThreeDigitNumber.
impl Add for ThreeDigitNumber {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Self::Output {
        let sum = self.value() + other.value();
        if sum > 999 {
            return ThreeDigitNumber::new_with_flag(sum - 1000, Some(Flag::OVERFLOW));
        }
        ThreeDigitNumber::new(sum)
    }
}

// Implement the AddAssign trait for ThreeDigitNumber.
impl AddAssign for ThreeDigitNumber {
    fn add_assign(&mut self, other: Self) {
        let result = *self + other;
        *self = result.unwrap();
    }
}

// Implement the Sub trait for ThreeDigitNumber.
impl Sub for ThreeDigitNumber {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Self::Output {
        let diff = self.value() - other.value();
        if diff < 0 {
            return ThreeDigitNumber::new_with_flag(diff.rem_euclid(1000), Some(Flag::NEG));
        }
        ThreeDigitNumber::new(diff)
    }
}

// Implement the SubAssign trait for ThreeDigitNumber.
impl SubAssign for ThreeDigitNumber {
    fn sub_assign(&mut self, other: Self) {
        let result = *self - other;
        *self = result.unwrap();
    }
}

// Display trait for easy printing.
impl fmt::Display for ThreeDigitNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:03}", self.value())
    }
}

// TwoDigitNumber is a 2-digit decimal number from 00-99
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TwoDigitNumber(u8, Option<Flag>);

impl TwoDigitNumber {
    pub fn new(value: u8) -> Result<Self, String> {
        if value <= 99 {
            Ok(TwoDigitNumber(value, None))
        } else {
            Err("Out of bounds: must be in the range 00-99".into())
        }
    }

    pub fn new_with_flag(value: u8, flag: Option<Flag>) -> Result<Self, String> {
        if value <= 99 {
            Ok(TwoDigitNumber(value, flag))
        } else {
            Err("Out of bounds: must be in the range 00-99".into())
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn flag(&self) -> Option<Flag> {
        self.1
    }
}

// Implement the Add trait for TwoDigitNumber.
impl Add for TwoDigitNumber {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Self::Output {
        let sum = self.value() + other.value();
        if sum > 99 {
            return TwoDigitNumber::new_with_flag(sum % 100, Some(Flag::OVERFLOW));
        }
        TwoDigitNumber::new(sum)
    }
}

// Implement the AddAssign trait for TwoDigitNumber.
impl AddAssign for TwoDigitNumber {
    fn add_assign(&mut self, other: Self) {
        let result = *self + other;
        *self = result.unwrap();
    }
}

// Display trait for easy printing.
impl fmt::Display for TwoDigitNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}", self.value())
    }
}
