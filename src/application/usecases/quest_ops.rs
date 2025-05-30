use crate::domain::{
    repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository},
    value_objects::quest_model::{AddQuestModel, EditQuestModel, QuestModel},
};
use anyhow::Result;
use std::sync::Arc;
pub struct QuestOpsUseCase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    quest_ops_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> QuestOpsUseCase<T1, T2>
where
    T1: QuestOpsRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_ops_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            quest_ops_repository,
            quest_viewing_repository,
        }
    }

    pub async fn add(
        &self,
        guild_commander_id: i32,
        add_quest_model: AddQuestModel,
    ) -> Result<i32> {
        let add_quest_entity = add_quest_model.to_entity(guild_commander_id);
        let quest_id = self.quest_ops_repository.add(add_quest_entity).await?;
        Ok(quest_id)
    }

    pub async fn edit(
        &self,
        quest_id: i32,
        guild_commander_id: i32,
        edit_quest_model: EditQuestModel,
    ) -> Result<i32> {
        let adventurer_count = self
            .quest_viewing_repository
            .adventurer_counting_by_quest_id(quest_id)
            .await?;

        if adventurer_count > 0 {
            return Err(anyhow::anyhow!("Quest is already started"));
        }

        let edit_quest_entity = edit_quest_model.to_entity(guild_commander_id);
        let quest_id = self
            .quest_ops_repository
            .edit(quest_id, edit_quest_entity)
            .await?;
        Ok(quest_id)
    }

    pub async fn remove(&self, quest_id: i32, guild_commander_id: i32) -> Result<()> {
        let adventurer_count = self
            .quest_viewing_repository
            .adventurer_counting_by_quest_id(quest_id)
            .await?;

        if adventurer_count > 0 {
            return Err(anyhow::anyhow!("Quest is already started"));
        }

        self.quest_ops_repository
            .remove(quest_id, guild_commander_id)
            .await?;
        Ok(())
    }
}
