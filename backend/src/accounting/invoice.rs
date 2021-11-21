use chrono::prelude::*;

use super::contact::Contact;
use super::product::Product;
use super::tax::Tax;
use crate::accounting::tax::TaxCalculation;
use crate::accounting::Account;

///
/// Invoice
/// Root Entity
///
pub struct Invoice {
    id: u32,
    number: String,
    status: InvoiceStatus,
    customer: Contact,
    date: DateTime<Utc>,
    due_date: DateTime<Utc>,
    notes: String,
    terms: String,
    tax_calculation: TaxCalculation,
    product_lines: Vec<ProductLine>,
    payments: Vec<InvoicePayment>,
}

///
/// Invoice Statuses
///
pub enum InvoiceStatus {
    Sent,
    Draft,
    Overdue,
    Paid,
    Void,
    PartiallyPaid,
}

///
/// Product Line
///
pub struct ProductLine {
    id: u32,
    description: Option<String>,
    price: f32,
    quantity: f32,
    product: Product,
    taxes: Vec<Tax>,
}

///
/// Invoice Payment
///
pub struct InvoicePayment {
    id: u32,
    date: DateTime<Utc>,
    bank_account: Account,
    amount: f32,
    description: String,
}

impl Invoice {
    fn new(
        id: u32,
        number: String,
        status: InvoiceStatus,
        customer: Contact,
        date: DateTime<Utc>,
        due_date: DateTime<Utc>,
        notes: String,
        terms: String,
        tax_calculation: TaxCalculation,
        product_lines: Vec<ProductLine>,
        payments: Vec<InvoicePayment>,
    ) -> Self {
        Invoice {
            id,
            number,
            status,
            customer,
            date,
            due_date,
            notes,
            terms,
            tax_calculation,
            product_lines,
            payments,
        }
    }

    pub fn new_open(
        id: u32,
        number: String,
        customer: Contact,
        date: DateTime<Utc>,
        due_date: DateTime<Utc>,
        notes: String,
        terms: String,
        tax_calculation: TaxCalculation,
        product_lines: Vec<ProductLine>,
        payments: Vec<InvoicePayment>,
    ) -> Self {
        Self::new(
            id,
            number,
            InvoiceStatus::Open,
            customer,
            date,
            due_date,
            notes,
            terms,
            tax_calculation,
            product_lines,
            payments,
        )
    }

    pub fn set_number(&mut self, number: String) {
        self.number = number;
    }

    pub fn set_status(&mut self, status: InvoiceStatus) {
        self.status = status
    }

    pub fn pay(&mut self) {
        self.status = InvoiceStatus::Paid;
    }
}
