use crate::model::Currency;
use chrono::NaiveDateTime;
use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum AccountType {
    Salary,
    Savings,
    Checking,
    Investiment,
    Stock,
    ExternalParty,
}

#[derive(Serialize, Deserialize)]
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

impl Display for AccountType {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            AccountType::Salary => write!(formatter, "SALARY"),
            AccountType::Savings => write!(formatter, "SAVINGS"),
            AccountType::Checking => write!(formatter, "CHECKING"),
            AccountType::Investiment => write!(formatter, "INVESTMENT"),
            AccountType::Stock => write!(formatter, "STOCK"),
            AccountType::ExternalParty => write!(formatter, "EXTERNAL_PARTY"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub bank_name: String,
    pub open_date: NaiveDateTime,
    pub close_date: NaiveDateTime,
    pub account_type: AccountType,
    pub balances: Vec<AccountBalance>,
    pub status: AccountStatus,
}

impl Display for AccountStatus {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            AccountStatus::Open => write!(formatter, "OPEN"),
            AccountStatus::Closed => write!(formatter, "CLOSED"),
            AccountStatus::NotInUse => write!(formatter, "NOT_IN_USE"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(AccountType::Salary, "SALARY".to_string(); "Should Return String for Salary")]
    #[test_case(AccountType::Savings, "SAVINGS".to_string(); "Should Return String for Savings")]
    #[test_case(AccountType::Checking, "CHECKING".to_string(); "Should Return String for Checking")]
    #[test_case(AccountType::Investiment, "INVESTMENT".to_string(); "Should Return String for Investiment")]
    #[test_case(AccountType::Stock, "STOCK".to_string(); "Should Return String for Stock")]
    #[test_case(AccountType::ExternalParty, "EXTERNAL_PARTY".to_string(); "Should Return String for ExternalParty")]
    fn should_return_string_representation_of_account_type(
        account_type: AccountType,
        expected_string: String,
    ) {
        assert_eq!(expected_string, account_type.to_string())
    }

    #[test_case(AccountStatus::Open, "OPEN".to_string(); "Should Return String for Open")]
    #[test_case(AccountStatus::Closed, "CLOSED".to_string(); "Should Return String for Closed")]
    #[test_case(AccountStatus::NotInUse, "NOT_IN_USE".to_string(); "Should Return String for NotInUse")]
    fn should_return_string_representation_of_account_status(
        account_status: AccountStatus,
        expected_string: String,
    ) {
        assert_eq!(expected_string, account_status.to_string())
    }
}
