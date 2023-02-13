pub mod account;

pub enum CurrencyCode {
    BRL,
}
pub struct Currency {
    pub code: CurrencyCode,
    pub value: f64,
}
