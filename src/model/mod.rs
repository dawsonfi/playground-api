use serde::{Deserialize, Serialize};

pub mod account;

#[derive(Serialize, Deserialize)]
pub enum CurrencyCode {
    BRL,
}

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub code: CurrencyCode,
    pub value: f64,
}
