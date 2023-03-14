use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod account;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum CurrencyCode {
    BRL,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Currency {
    pub code: CurrencyCode,
    pub value: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseEnumError {
    pub message: String,
}

impl Display for CurrencyCode {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            CurrencyCode::BRL => write!(formatter, "BRL"),
        }
    }
}

impl FromStr for CurrencyCode {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "BRL" => Ok(CurrencyCode::BRL),
            _ => Err(ParseEnumError {
                message: format!("Invalid CurrencyCode {value}"),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(CurrencyCode::BRL, "BRL".to_string())]
    fn should_return_string_representation_of_currency_code(
        #[case] currency_code: CurrencyCode,
        #[case] expected_string: String,
    ) {
        assert_eq!(expected_string, currency_code.to_string())
    }

    #[rstest]
    #[case(CurrencyCode::BRL, "BRL")]
    fn should_return_enum_from_string_of_currency_code(
        #[case] expected_currency_code: CurrencyCode,
        #[case] enum_string: &str,
    ) {
        let currency_enum = CurrencyCode::from_str(enum_string);
        assert!(currency_enum.is_ok());
        assert_eq!(expected_currency_code, currency_enum.unwrap());
    }

    #[test]
    fn should_return_err_when_currency_code_does_not_exist() {
        let currency_code = CurrencyCode::from_str("batata");

        assert!(currency_code.is_err());
    }

    #[test]
    fn currency_code_should_be_thread_safe() {
        is_thread_safe::<CurrencyCode>();
    }

    #[test]
    fn currency_should_be_thread_safe() {
        is_thread_safe::<Currency>();
    }

    #[test]
    fn parse_enum_error_should_be_thread_safe() {
        is_thread_safe::<ParseEnumError>();
    }

    fn is_thread_safe<T: Sized + Send + Sync + Unpin>() {}
}
