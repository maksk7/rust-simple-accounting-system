use chrono::prelude::*;

use super::currencies::Currency;

pub struct Account {
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

impl Account {
    fn new(
        name: String,
        account_type: AccountType,
        currency: Currency,
        code: String,
        is_active: bool,
        opened_at: Option<DateTime<Utc>>,
    ) -> Self {
        let opened_at = opened_at.unwrap_or(Utc::now());

        Account {
            name,
            account_type,
            currency,
            code,
            is_active,
            opened_at,
        }
    }

    fn new_bank_account(
        name: String,
        currency: Currency,
        code: String,
        is_active: bool,
        opened_at: Option<DateTime<Utc>>,
    ) -> Account {
        Self::new(
            name,
            AccountType::Bank,
            currency,
            code,
            is_active,
            opened_at,
        )
    }

}
