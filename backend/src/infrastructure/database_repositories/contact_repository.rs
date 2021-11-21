use crate::accounting::{Contact, Address, ContactType};
use crate::core::Repository;
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

struct ContactRepository {
    db_pool: sqlx::PgPool,
}

#[derive(Debug, FromRow)]
pub struct ContactDTO {
    id: u32,
    email: String,
    first_name: String,
    last_name: String,
    middle_name: Option<String>,
    contact_type: ContactType,
    billing_addresses: Vec<Address>,
    shipping_addresses: Vec<Address>,
    created_at: DateTime<Utc>,
}

impl From<ContactDTO> for Contact {
    fn from(dto: ContactDTO) -> Self {
        Self {
            id: dto.id,
            email: dto.email,
            first_name: dto.first_name,
            last_name: dto.last_name,
            middle_name: dto.middle_name,
            contact_type: dto.contact_type,
            billing_addresses: dto.billing_addresses,
            shipping_addresses: dto.shipping_addresses,
            created_at: dto.created_at
        }
    }
}

impl Repository<Contact> for ContactRepository {
    async fn find_one_by_id(&self, id: f32) -> Result<Ok(Contact), Err(E)> {
        let result: ContactDTO = sqlx::query_as("SELECT * FROM contacts WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(Contact::from(result))
    }

    async fn find_one_by(&self, fields: HashMap<String, String>) -> Result<Contact, sqlx::Error> {
        todo!()
    }

    async fn find_by(&self, fields: HashMap<String, String>) -> Result<Vec<Contact>, sqlx::Error> {
        todo!()
    }

    async fn add(&self, entity: Contact) -> Result<Ok(), error> {
        todo!()
    }

    async fn delete(&self, entity: Contact) -> Result<Ok(), error> {
        todo!()
    }

    async fn edit(&self, entity: Contact) -> Result<Ok(Contact), error> {
        todo!()
    }
}
