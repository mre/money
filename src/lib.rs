#![feature(slice_patterns)]

#[macro_use]
extern crate failure;

use std::num::ParseFloatError;

#[derive(Debug, Fail, PartialEq)]
enum MoneyError {
    #[fail(display = "Invalid input: {}", _0)] ParseAmount(ParseFloatError),
    #[fail(display = "{}", _0)] ParseCurrency(String),
    #[fail(display = "{}", _0)] ParseFormatting(String),
}

impl From<ParseFloatError> for MoneyError {
    fn from(e: ParseFloatError) -> Self {
        MoneyError::ParseAmount(e)
    }
}

#[derive(Debug, PartialEq)]
struct Money {
    amount: f32,
    currency: Currency,
}

impl Money {
    fn new(amount: f32, currency: Currency) -> Self {
        Money { amount, currency }
    }
}

impl std::str::FromStr for Money {
    type Err = MoneyError;

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

#[derive(Debug, PartialEq)]
enum Currency {
    Dollar,
    Euro,
}

impl std::str::FromStr for Currency {
    type Err = MoneyError;

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
    fn it_works() {
        assert_eq!(
            "140.01".parse::<Money>(),
            Err(MoneyError::ParseFormatting(
                "Expecting amount and currency".into()
            ))
        );

        let result = "OneMillion Euro".parse::<Money>();
        assert!(result.is_err());
        assert_eq!(
            "100 Euro".parse::<Money>(),
            Ok(Money {
                amount: 100.0,
                currency: Currency::Euro,
            })
        );
    }
}
