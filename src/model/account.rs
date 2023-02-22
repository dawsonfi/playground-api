use crate::model::{Currency, ParseEnumError};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AccountType {
    Salary,
    Savings,
    Checking,
    Investment,
    Stock,
    ExternalParty,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum AccountStatus {
    Open,
    Closed,
    NotInUse,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalance {
    pub date: NaiveDateTime,
    pub balance: Currency,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub bank_name: String,
    pub open_date: NaiveDateTime,
    pub close_date: Option<NaiveDateTime>,
    pub account_type: AccountType,
    pub balances: Vec<AccountBalance>,
    pub status: AccountStatus,
}

impl Display for AccountType {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            AccountType::Salary => write!(formatter, "SALARY"),
            AccountType::Savings => write!(formatter, "SAVINGS"),
            AccountType::Checking => write!(formatter, "CHECKING"),
            AccountType::Investment => write!(formatter, "INVESTMENT"),
            AccountType::Stock => write!(formatter, "STOCK"),
            AccountType::ExternalParty => write!(formatter, "EXTERNAL_PARTY"),
        }
    }
}

impl FromStr for AccountType {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "SALARY" => Ok(AccountType::Salary),
            "SAVINGS" => Ok(AccountType::Savings),
            "CHECKING" => Ok(AccountType::Checking),
            "INVESTMENT" => Ok(AccountType::Investment),
            "STOCK" => Ok(AccountType::Stock),
            "EXTERNAL_PARTY" => Ok(AccountType::ExternalParty),
            _ => Err(ParseEnumError {
                message: format!("Invalid AccountType {value}"),
            }),
        }
    }
}

impl Display for AccountStatus {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        match self {
            AccountStatus::Open => write!(formatter, "OPEN"),
            AccountStatus::Closed => write!(formatter, "CLOSED"),
            AccountStatus::NotInUse => write!(formatter, "NOT_IN_USE"),
        }
    }
}

impl FromStr for AccountStatus {
    type Err = ParseEnumError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "OPEN" => Ok(AccountStatus::Open),
            "CLOSED" => Ok(AccountStatus::Closed),
            "NOT_IN_USE" => Ok(AccountStatus::NotInUse),
            _ => Err(ParseEnumError {
                message: format!("Invalid AccountStatus {value}"),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(AccountType::Salary, "SALARY".to_string())]
    #[case(AccountType::Savings, "SAVINGS".to_string())]
    #[case(AccountType::Checking, "CHECKING".to_string())]
    #[case(AccountType::Investment, "INVESTMENT".to_string())]
    #[case(AccountType::Stock, "STOCK".to_string())]
    #[case(AccountType::ExternalParty, "EXTERNAL_PARTY".to_string())]
    fn should_return_string_representation_of_account_type(
        #[case] account_type: AccountType,
        #[case] expected_string: String,
    ) {
        assert_eq!(expected_string, account_type.to_string())
    }

    #[rstest]
    #[case(AccountType::Salary, "SALARY")]
    #[case(AccountType::Savings, "SAVINGS")]
    #[case(AccountType::Checking, "CHECKING")]
    #[case(AccountType::Investment, "INVESTMENT")]
    #[case(AccountType::Stock, "STOCK")]
    #[case(AccountType::ExternalParty, "EXTERNAL_PARTY")]
    fn should_return_enum_from_string_of_account_type(
        #[case] expected_account_type: AccountType,
        #[case] enum_string: &str,
    ) {
        let account_type = AccountType::from_str(enum_string);
        assert!(account_type.is_ok());
        assert_eq!(expected_account_type, account_type.unwrap());
    }

    #[test]
    fn should_return_err_when_account_type_does_not_exist() {
        let account_type = AccountType::from_str("batata");

        assert!(account_type.is_err());
    }

    #[rstest]
    #[case(AccountStatus::Open, "OPEN".to_string())]
    #[case(AccountStatus::Closed, "CLOSED".to_string())]
    #[case(AccountStatus::NotInUse, "NOT_IN_USE".to_string())]
    fn should_return_string_representation_of_account_status(
        #[case] account_status: AccountStatus,
        #[case] expected_string: String,
    ) {
        assert_eq!(expected_string, account_status.to_string())
    }

    #[rstest]
    #[case(AccountStatus::Open, "OPEN")]
    #[case(AccountStatus::Closed, "CLOSED")]
    #[case(AccountStatus::NotInUse, "NOT_IN_USE")]
    fn should_return_enum_from_string_of_account_status(
        #[case] expected_account_status: AccountStatus,
        #[case] enum_string: &str,
    ) {
        let account_enum = AccountStatus::from_str(enum_string);
        assert!(account_enum.is_ok());
        assert_eq!(expected_account_status, account_enum.unwrap());
    }

    #[test]
    fn should_return_err_when_account_status_does_not_exist() {
        let account_status = AccountStatus::from_str("batata");

        assert!(account_status.is_err());
    }
}
