use crate::core::Entity;
use async_trait::async_trait;

pub trait Repository {
    type E: Entity;

    async fn save(&entity: impl Entity) -> Result<Ok(), Err>;
    fn get_all() -> Vec<E>;
}
