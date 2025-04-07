use anyhow::Result;
use axum::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait JourneyLedgerRepository {
    async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<bool>;
    async fn to_completed(&self, quest_id: i32, guild_commander_id: i32) -> Result<bool>;
    async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<bool>;
}
