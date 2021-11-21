use crate::accounting::account::AccountType;
use chrono::{DateTime, Utc};
use std::fmt;

use super::account::Account;

pub struct Transfer {
    id: u32,
    from_account: Account,
    to_account: Account,
    amount: f32,
    description: String,
    date: DateTime<Utc>,
}

impl Transfer {
    pub fn new(
        id: u32,
        from_account: Account,
        to_account: Account,
        amount: f32,
        description: String,
        date: DateTime<Utc>,
    ) -> Self {
        Transfer {
            id,
            from_account,
            to_account,
            amount,
            description,
            date,
        }
    }

    pub fn get_from_account(&self) -> &Account {
        &self.from_account
    }

    pub fn set_from_account(
        &mut self,
        from_account: Account,
    ) -> Result<Ok(), AccountHasInvalidType> {
        if from_account.get_account_type() != AccountType::Bank {
            Err(AccountHasInvalidType)
        }

        self.from_account = from_account;

        Ok(())
    }

    pub fn get_to_account(&self) -> &Account {
        &self.to_account
    }

    pub fn set_to_account(&mut self, to_account: Account) -> Result<Ok(), AccountHasInvalidType> {
        if to_account.get_account_type() != AccountType::Bank {
            Err(AccountHasInvalidType)
        }

        self.to_account = to_account;

        Ok(())
    }

    pub fn get_amount(&self) -> &f32 {
        &self.amount
    }

    pub fn set_amount(&mut self, amount: f32) -> Result<Ok(), Amount> {
        if amount <= 0f32 {
            Err(AmountLessOrEqualZero)
        }

        self.amount = amount;
        Ok(())
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn set_date(&mut self, date: DateTime<Utc>) {
        self.date = date;
    }
}

#[derive(Debug, Clone)]
struct AccountHasInvalidTypeError;

impl fmt::Display for AccountHasInvalidType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Account has invalid type(only Bank accounts can be entered)."
        )
    }
}

#[derive(Debug, Clone)]
struct AmountLessOrEqualZero;

impl fmt::Display for AmountLessOrEqualZero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Amount less or equal to zero.")
    }
}
