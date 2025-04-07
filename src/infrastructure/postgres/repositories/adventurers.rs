use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::{
        entities::adventurers::RegisterAdventurerEntity,
        repositories::adventurers::AdventurerRepository,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct AdventurerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AdventurerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AdventurerRepository for AdventurerPostgres {
    async fn register(&self, register_adventurer_entity: RegisterAdventurerEntity) -> Result<i32> {
        unimplemented!();
    }
    async fn find_by_username(&self, username: String) -> Result<RegisterAdventurerEntity> {
        unimplemented!();
    }
}
