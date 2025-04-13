use std::sync::Arc;

use crate::domain::{
    repositories::quest_viewing::QuestViewingRepository,
    value_objects::{board_checking_filter::BoardCheckingFilter, quest_model::QuestModel},
};
use anyhow::{Ok, Result};
pub struct QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    quest_viewing_repository: Arc<T>,
}

impl<T> QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_viewing_repository: Arc<T>) -> Self {
        Self {
            quest_viewing_repository,
        }
    }

    pub async fn view_details(&self, quest_id: i32) -> Result<QuestModel> {
        let result = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurer_count = self
            .quest_viewing_repository
            .adventurer_counting_by_quest_id(quest_id)
            .await?;

        Ok(result.to_model(adventurer_count))
    }

    pub async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>> {
        let results = self.quest_viewing_repository.board_checking(filter).await?;

        let mut quest_model: Vec<QuestModel> = Vec::new();

        for quest in results.into_iter() {
            let adventurer_count = self
                .quest_viewing_repository
                .adventurer_counting_by_quest_id(quest.id)
                .await?;

            quest_model.push(quest.to_model(adventurer_count));
        }

        Ok(quest_model)
    }
}
