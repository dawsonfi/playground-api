use chrono::NaiveDateTime;
use crate::model::Currency;

pub enum AccountType {
    Salary,
    Savings,
    Checking,
    Investiment,
    Stock,
    ExternalParty
}

pub enum AccountStatus {
    Open,
    Closed,
    NotInUse
}

pub struct AccountBalance {
    pub date: NaiveDateTime,
    pub balance: Currency
}

pub struct Account {
    pub id: String,
    pub name: String,
    pub bank_name: String,
    pub open_date: NaiveDateTime,
    pub close_date: NaiveDateTime,
    pub account_type: AccountType,
    pub balances: Vec<AccountBalance>,
    pub status: AccountStatus
}