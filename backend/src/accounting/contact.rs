use chrono::prelude::*;

// Root entity
pub struct Contact {
    id: u32,
    email: String,
    first_name: String,
    last_name: String,
    middle_name: Option<String>,
    contact_type: Type,
    billing_addresses: Option<Vec<Address>>,
    shipping_addresses: Option<Vec<Address>>,
    registered_at: DateTime<Utc>,
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
    fn new(
        id: u32,
        email: String,
        first_name: String,
        last_name: String,
        middle_name: Option<String>,
        contact_type: ContactType,
        billing_addresses: Option<Vec<Address>>,
        shipping_addresses: Option<Vec<Address>>,
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
            registered_at: Utc::now(),
        }
    }

    pub fn new_customer(
        id: u32,
        email: String,
        first_name: String,
        last_name: String,
        middle_name: Option<String>,
        billing_addresses: Option<Vec<Address>>,
        shipping_addresses: Option<Vec<Address>>,
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
        billing_addresses: Option<Vec<Address>>,
        shipping_addresses: Option<Vec<Address>>,
    ) -> Self {
        Self::new(
            id,
            email,
            first_name,
            last_name,
            middle_name,
            ContactType::Vendor,
            billing_addresses,
            shipping_addresses,
        )
    }

    pub fn change_email(&mut self, email: String) -> Result<Ok(), Err(E)> {
        if is_valid_email(&email) {
            self.email = email;
            Ok(());
        }

        Err(String::from("Email is not valid"))
    }

    pub fn change_first_name(&mut self, first_name: String) {
        self.first_name = first_name;
    }

    pub fn change_last_name(&mut self, last_name: String) {
        self.last_name = last_name;
    }

    pub fn change_middle_name(&mut self, middle_name: String) {
        self.middle_name = Some(middle_name);
    }

    pub fn add_billing_address(&mut self) {
        if Some(i) == self.billing_addresses {}
    }

    pub fn add_shipping_address(&mut self) {}
}
