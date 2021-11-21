use crate::accounting::account::AccountType;
use crate::accounting::errors::AccountingError;
use crate::accounting::{Account, Contact, Currency, Tax};
use chrono::{DateTime, Utc};

pub struct Expense {
    bank_account: Account,
    amount: f32,
    taxes: Vec<Tax>,
    description: String,
    date: DateTime<Utc>,
    customer: Contact,
    currency: Currency,
    rate: f32,
}

impl Expense {
    fn new(
        bank_account: Account,
        amount: f32,
        taxes: Vec<Tax>,
        description: String,
        date: DateTime<Utc>,
        customer: Contact,
        currency: Currency,
        rate: f32,
    ) -> Result<Self, Err()> {
        if bank_account.get_account_type() != AccountType::Bank {
            Err(AccountingError::InvalidAccountType)
        }

        Ok(Expense {
            bank_account,
            amount,
            taxes,
            description,
            date,
            customer,
            currency,
            rate,
        })
    }
}
