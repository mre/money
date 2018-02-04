//! # Money
//!
//! Sample code for my talk [*Idiomatic Rust*](https://fosdem.org/2018/schedule/event/rust_idiomatic/), which I gave at FOSDEM 2018.
//!
//! This library shows how to create ergonomic value objects in Rust.
//! The point of this case-study is to show common Rust idiomas and patterns.
//! *A word of caution:* this code is for educational purposes only.
//! Generally, it's a bad idea to represent a monetary value as float.
//! If you want to implement something similar for real-world use-cases, read
//! [this](https://deque.blog/2017/08/17/a-study-of-4-money-class-designs-featuring-martin-fowler-kent-beck-and-ward-cunningham-implementations/) first.
//!
//! Only works on nightly Rust for now [until slice patterns are stabilized](https://github.com/rust-lang/rust/issues/23121).

// We use a nightly feature for making our code
// a little easier on the eye.
// This can be removed, as soon as
// [slice patterns are stabilized](https://github.com/rust-lang/rust/issues/23121)
#![feature(slice_patterns)]

// Failure is a crate for making custom error types
// easier to write and integrate with existing errors.
#[macro_use]
extern crate failure;

// This error will be thrown, when our monetary value cannot be parsed
// (e.g if it's not a floating point number).
use std::num::ParseFloatError;

/// Our custom error type.
#[derive(Debug, Fail, PartialEq)]
pub enum MoneyError {
    /// Error while parsing the amount as float
    #[fail(display = "Invalid input: {}", _0)]
    ParseAmount(ParseFloatError),
    /// Error while parsing currency
    #[fail(display = "{}", _0)]
    ParseCurrency(String),
    /// General formatting error (e.g. input string does not consist of amount and currency)
    #[fail(display = "{}", _0)]
    ParseFormatting(String),
}

/// A conversion from `std::num::ParseFloatError`
/// into our custom MoneyError type.
impl From<ParseFloatError> for MoneyError {
    fn from(e: ParseFloatError) -> Self {
        MoneyError::ParseAmount(e)
    }
}

#[derive(Debug, PartialEq)]
/// Our Money type.
/// We derive `PartialEq` for comparing objects
pub struct Money {
    amount: f32,
    currency: Currency,
}

/// Money is our core library type, which consists of an
/// amount and a currency.
/// Example:
/// ```
/// let cash = "10.12 $".parse::<Money>();
/// ```
impl Money {
    fn new(amount: f32, currency: Currency) -> Self {
        Money { amount, currency }
    }
}

/// We implement `std::str::FromStr` for converting
/// a string into Money.
impl std::str::FromStr for Money {
    type Err = MoneyError;

    /// Right now, we are using a nightly feature for string to type conversion.
    /// See [slice patterns](https://github.com/rust-lang/rust/issues/23121).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts[..] {
            [amount, currency] => Ok(Money::new(amount.parse()?, currency.parse()?)),
            _ => Err(MoneyError::ParseFormatting(
                "Expecting amount and currency".into(),
            )),
        }
    }
}

/// Supported currencies
#[derive(Debug, PartialEq)]
enum Currency {
    Dollar,
    Euro,
}

impl std::str::FromStr for Currency {
    type Err = MoneyError;

    /// Match based on the input string and return the correct
    /// `Currency` type.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "dollar" | "$" => Ok(Currency::Dollar),
            "euro" | "eur" | "€" => Ok(Currency::Euro),
            _ => Err(MoneyError::ParseCurrency("Unknown currency".into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errors() {
        assert_eq!(
            "140.01".parse::<Money>(),
            Err(MoneyError::ParseFormatting(
                "Expecting amount and currency".into()
            ))
        );

        let result = "OneMillion Euro".parse::<Money>();
        assert!(result.is_err());
    }

    #[test]
    fn test_successful_parsing() {
        let testcases = vec![
            (
                "100 Euro",
                Money {
                    amount: 100.0,
                    currency: Currency::Euro,
                },
            ),
            (
                "10 $",
                Money {
                    amount: 10.0,
                    currency: Currency::Dollar,
                },
            ),
            (
                "42.4 DOLLAR",
                Money {
                    amount: 42.4,
                    currency: Currency::Dollar,
                },
            ),
        ];

        for (input, output) in testcases {
            assert_eq!(input.parse::<Money>(), Ok(output));
        }
    }
}
