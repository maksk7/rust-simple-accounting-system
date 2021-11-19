use chrono::prelude::*;

use super::contact::Contact;
use super::product::Product;
use super::tax::Tax;

pub struct Invoice {
    id: u32,
    number: String,
    status: InvoiceStatus,
    vendor: Contact,
    date: DateTime<Utc>,
    due_date: DateTime<Utc>,
    notes: Option<String>,
    terms: Option<String>,
    product_lines: Vec<ProductLine>,
    payments: Vec<InvoicePayment>
}

pub enum InvoiceStatus {
    Paid,
    Open,
    Overdue,
    Void,
    PartiallyPaid
}

pub struct ProductLine {
    id: u32,
    description: Option<String>,
    price: f32,
    quantity: f32,
    product: Product,
    taxes: Vec<Tax>
}

pub struct InvoicePayment {
    date: DateTime<Utc>,

}

impl Invoice {
    pub fn pay(&self) {
        self.status = InvoiceStatus::Paid;

        self
    }
}