use std::fmt::Debug;
use std::ops::Add;
use std::string::String as RustString;

use crate::prelude::*;

#[derive(PartialEq, Debug)]
pub struct String(RustString);

//
// Type trait
impl Type<RustString> for String {
    fn value(self) -> RustString {
        self.0
    }
}

//
// From Trait

// String
impl From<RustString> for String {
    fn from(a: RustString) -> Self {
        Self(a)
    }
}

impl From<&'static str> for String {
    fn from(a: &'static str) -> Self {
        Self(RustString::from(a))
    }
}

// Integers
impl From<i8> for String {
    fn from(a: i8) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

impl From<i16> for String {
    fn from(a: i16) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}


impl From<i32> for String {
    fn from(a: i32) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}


impl From<i64> for String {
    fn from(a: i64) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

impl From<i128> for String {
    fn from(a: i128) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

impl From<isize> for String {
    fn from(a: isize) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

// Unsigned Integers

impl From<u8> for String {
    fn from(a: u8) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

impl From<u16> for String {
    fn from(a: u16) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}


impl From<u32> for String {
    fn from(a: u32) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}


impl From<u64> for String {
    fn from(a: u64) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

impl From<u128> for String {
    fn from(a: u128) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

impl From<usize> for String {
    fn from(a: usize) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

// Floating point
impl From<f32> for String {
    fn from(a: f32) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

impl From<f64> for String {
    fn from(a: f64) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

// Boolean
impl From<bool> for String {
    fn from(a: bool) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

// Char
impl From<char> for String {
    fn from(a: char) -> Self {
        Self(RustString::from(format!("{}", a)))
    }
}

//
// Into
impl Into<RustString> for String {
    fn into(self) -> RustString {
        self.value()
    }
}

//
// Add
impl Add<String> for String {
    type Output = String;

    fn add(self, rhs: String) -> Self::Output {
        Self(self.0 + rhs.0.as_ref())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_type_trait() {
        let s = String::from("Test");
        assert_eq!(s.value(), RustString::from("Test"));
    }

    #[test]
    fn string_from_string() {
        let s = String::from("Test");
        assert_eq!(s.value(), RustString::from("Test"));
        let s = String::from(RustString::from("Test"));
        assert_eq!(s.value(), RustString::from("Test"));
        let s = String::from(String::from("Test"));
        assert_eq!(s.value(), RustString::from("Test"));
    }

    #[test]
    fn string_from_int() {
        assert_eq!(String::from(5i8).value(), RustString::from("5"));
        assert_eq!(String::from(5i16).value(), RustString::from("5"));
        assert_eq!(String::from(5i32).value(), RustString::from("5"));
        assert_eq!(String::from(5i64).value(), RustString::from("5"));
        assert_eq!(String::from(5i128).value(), RustString::from("5"));
        assert_eq!(String::from(5isize).value(), RustString::from("5"));
        assert_eq!(String::from(5u8).value(), RustString::from("5"));
        assert_eq!(String::from(5u16).value(), RustString::from("5"));
        assert_eq!(String::from(5u32).value(), RustString::from("5"));
        assert_eq!(String::from(5u64).value(), RustString::from("5"));
        assert_eq!(String::from(5u128).value(), RustString::from("5"));
        assert_eq!(String::from(5usize).value(), RustString::from("5"));
    }

    #[test]
    fn string_from_double() {
        assert_eq!(String::from(5.1f32).value(), RustString::from("5.1"));
        assert_eq!(String::from(5.1f64).value(), RustString::from("5.1"));
    }

    #[test]
    fn string_from_bool() {
        assert_eq!(String::from(true).value(), RustString::from("true"));
        assert_eq!(String::from(false).value(), RustString::from("false"));
    }

    #[test]
    fn string_from_char() {
        assert_eq!(String::from('t').value(), RustString::from("t"));
    }
}