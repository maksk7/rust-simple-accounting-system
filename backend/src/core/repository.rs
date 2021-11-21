use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;

use crate::core::Entity;

#[async_trait]
pub trait Repository<T>
where
    T: Entity,
{
    async fn find_one_by_id(&self, id: f32) -> Result<Ok(T), error>;
    async fn find_one_by(&self, field: HashMap<String, String>) -> Result<Ok(T), error>;
    async fn find_by(&self, field: HashMap<String, String>) -> Result<Ok(Vec<T>), error>;
    async fn add(&self, entity: T) -> Result<Ok(), error>;
    async fn delete(&self, entity: T) -> Result<Ok(), error>;
    async fn edit(&self, entity: T) -> Result<Ok(T), error>;
}
