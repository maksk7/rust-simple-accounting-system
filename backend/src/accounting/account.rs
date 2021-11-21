use chrono::prelude::*;

use super::currencies::Currency;

pub struct Account {
    id: u32,
    name: String,
    account_type: AccountType,
    currency: Currency,
    code: String,
    is_active: bool,
    opened_at: DateTime<Utc>,
}

pub enum AccountType {
    Bank,
    OtherAsset,
    OtherCurrentAsset,
    Cash,
    FixedAsset,
    OtherCurrentLiability,
    CreditCard,
    LongTermLiability,
    OtherLiability,
    Equity,
    Income,
    OtherIncome,
    Expense,
    CostOfGoodsSold,
    OtherExpense,
    AccountReceivable,
    AccountPayable,
}

pub enum AccountError {}

impl Account {
    pub fn new(
        id: u32,
        name: String,
        account_type: AccountType,
        currency: Currency,
        code: String,
        is_active: bool,
        opened_at: Option<DateTime<Utc>>,
    ) -> Self {
        let opened_at = opened_at.unwrap_or(Utc::now());

        Account {
            id,
            name,
            account_type,
            currency,
            code,
            is_active,
            opened_at,
        }
    }

    pub fn new_bank_account(
        id: u32,
        name: String,
        currency: Currency,
        code: String,
        is_active: bool,
        opened_at: Option<DateTime<Utc>>,
    ) -> Account {
        Self::new(
            id,
            name,
            AccountType::Bank,
            currency,
            code,
            is_active,
            opened_at,
        )
    }

    pub fn new_active_account(
        id: u32,
        name: String,
        account_type: AccountType,
        currency: Currency,
        code: String,
        opened_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self::new(id, name, account_type, currency, code, true, opened_at)
    }

    pub fn new_archived_account(
        id: u32,
        name: String,
        account_type: AccountType,
        currency: Currency,
        code: String,
        opened_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self::new(id, name, account_type, currency, code, false, opened_at)
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_account_type(&self) -> &AccountType {
        &self.account_type
    }

    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }

    pub fn set_currency(&mut self, currency: Currency) {
        self.currency = currency;
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn set_code(&mut self, code: String) {
        self.code = code;
    }

    pub fn is_active(&self) -> &bool {
        &self.is_active
    }

    pub fn activate(&mut self) {
        self.is_active = true
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}
