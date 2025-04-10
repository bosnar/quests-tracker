use anyhow::Result;
use std::sync::Arc;

use crate::{
    domain::{
        repositories::adventurers::AdventurerRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::argon2_hashing,
};

#[derive(Debug, Clone)]
pub struct AdventurersUseCase<T>
where
    T: AdventurerRepository + Send + Sync, // สามารถส่งของข้าม thread ได้, ทำงานพร้อมกันหลายๆ thread ได้
{
    adventurer_repository: Arc<T>,
}

impl<T> AdventurersUseCase<T>
where
    T: AdventurerRepository + Send + Sync,
{
    pub fn new(adventurer_repository: Arc<T>) -> Self {
        Self {
            adventurer_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_adventurer_model: RegisterAdventurerModel,
    ) -> Result<i32> {
        let hashed_password = argon2_hashing::hash(register_adventurer_model.password)?;

        register_adventurer_model.password = hashed_password;

        let adventurer_id = self
            .adventurer_repository
            .register(register_adventurer_model.to_entity())
            .await?;

        Ok(adventurer_id)
    }
}
