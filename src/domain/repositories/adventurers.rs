use crate::domain::entities::adventurers::RegisterAdventurerEntity;
use anyhow::Result;
use axum::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait AdventurerRepository {
    async fn register(&self, adventurer: RegisterAdventurerEntity) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<RegisterAdventurerEntity>;
}
