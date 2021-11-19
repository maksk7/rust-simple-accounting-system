use chrono::prelude::*;

use super::contact::Contact;
use super::product::Product;
use super::tax::Tax;

pub struct PurchaseOrder {
    id: u32,
    number: String,
    status: PurchaseOrderStatus,
    vendor: Contact,
    date: DateTime<Utc>,
    due_date: DateTime<Utc>,
    notes: Option<String>,
    terms: Option<String>,
    product_lines: Vec<ProductLine>,
}

pub enum PurchaseOrderStatus {
    Paid,
    Open,
    Overdue,
    Void,
    PartiallyPaid,
}

pub struct ProductLine {
    id: u32,
    description: Option<String>,
    price: f32,
    quantity: f32,
    product: Product,
    taxes: Vec<Tax>,
}
