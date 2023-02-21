use crate::model::Currency;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize)]
pub enum AccountType {
    Salary,
    Savings,
    Checking,
    Investment,
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
            AccountType::Investment => write!(formatter, "INVESTMENT"),
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
    pub close_date: Option<NaiveDateTime>,
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
    #[case(AccountStatus::Open, "OPEN".to_string())]
    #[case(AccountStatus::Closed, "CLOSED".to_string())]
    #[case(AccountStatus::NotInUse, "NOT_IN_USE".to_string())]
    fn should_return_string_representation_of_account_status(
        #[case] account_status: AccountStatus,
        #[case] expected_string: String,
    ) {
        assert_eq!(expected_string, account_status.to_string())
    }
}
