use chrono::prelude::*;

// Root entity
pub struct Contact {
    pub id: u32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub contact_type: ContactType,
    pub billing_addresses: Vec<Address>,
    pub shipping_addresses: Vec<Address>,
    pub created_at: DateTime<Utc>,
}

pub enum ContactType {
    Customer,
    Vendor,
}

pub struct Address {
    id: u32,
    street: String,
    zip: String,
    city: String,
    region: String,
    country: String,
}

impl Contact {
    pub fn new(
        id: u32,
        email: String,
        first_name: String,
        last_name: String,
        middle_name: Option<String>,
        contact_type: ContactType,
        billing_addresses: Vec<Address>,
        shipping_addresses: Vec<Address>,
    ) -> Self {
        Contact {
            id,
            email,
            first_name,
            last_name,
            middle_name,
            contact_type,
            billing_addresses,
            shipping_addresses,
            created_at: Utc::now(),
        }
    }

    pub fn new_customer(
        id: u32,
        email: String,
        first_name: String,
        last_name: String,
        middle_name: Option<String>,
        billing_addresses: Vec<Address>,
        shipping_addresses: Vec<Address>,
    ) -> Result<Self, Err(E)> {
        if !is_valid_email(&email) {
            Err(String::from("Email is not valid"))
        }

        Ok(Self::new(
            id,
            email,
            first_name,
            last_name,
            middle_name,
            ContactType::Customer,
            billing_addresses,
            shipping_addresses,
        ))
    }

    pub fn new_vendor(
        id: u32,
        email: String,
        first_name: String,
        last_name: String,
        middle_name: Option<String>,
        billing_addresses: Vec<Address>,
        shipping_addresses: Vec<Address>,
    ) -> Result<Self, Err(E)> {
        if !is_valid_email(&email) {
            Err(String::from("Email is not valid"))
        }

        Ok(Self::new(
            id,
            email,
            first_name,
            last_name,
            middle_name,
            ContactType::Vendor,
            billing_addresses,
            shipping_addresses,
        ))
    }

    pub fn set_email(&mut self, email: String) -> Result<Ok(), Err(E)> {
        if is_valid_email(&email) {
            self.email = email;
            Ok(());
        }

        Err(String::from("Email is not valid"))
    }

    pub fn set_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    pub fn set_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    pub fn set_middle_name(&mut self, middle_name: String) {
        self.middle_name = Some(middle_name);
    }

    pub fn add_billing_address(&mut self, billing_address: Address) {
        self.billing_addresses.push(billing_address);
    }

    pub fn add_shipping_address(&mut self, shipping_address: Address) {
        self.shipping_addresses.push(shipping_address);
    }
}
