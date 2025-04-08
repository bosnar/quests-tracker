use anyhow::Result;
use std::sync::Arc;

use crate::domain::{
    repositories::guild_commanders::GuildCommanderRepository,
    value_objects::guild_commander_model::RegisterGuildCommanderModel,
};

#[derive(Debug, Clone)]
pub struct GuildCommandersUseCase<T>
where
    T: GuildCommanderRepository + Send + Sync,
{
    guild_commanders_repository: Arc<T>,
}

impl<T> GuildCommandersUseCase<T>
where
    T: GuildCommanderRepository + Send + Sync,
{
    pub fn new(guild_commanders_repository: Arc<T>) -> Self {
        Self {
            guild_commanders_repository,
        }
    }

    pub async fn register(
        &self,
        register_guild_commander_model: RegisterGuildCommanderModel,
    ) -> Result<i32> {
        unimplemented!()
    }
}
