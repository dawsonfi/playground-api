use crate::model::account::{Account, AccountStatus, AccountType};
use crate::repository::account_repository::AccountRepository;
use aws_config::SdkConfig;
use std::error::Error;

pub struct AccountService {
    repository: AccountRepository,
}

impl AccountService {
    pub fn new(config: &SdkConfig) -> Self {
        AccountService {
            repository: AccountRepository::new(config),
        }
    }

    pub async fn list_accounts(
        &self,
        account_type: Option<AccountType>,
        account_status: Option<AccountStatus>,
    ) -> Result<Vec<Account>, Box<dyn Error>> {
        self.repository
            .list_accounts(account_type, account_status)
            .await
    }
}
