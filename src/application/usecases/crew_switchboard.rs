use anyhow::{Ok, Result};
use std::sync::Arc;

use crate::domain::{
    repositories::{
        crew_switchboard::CrewSwitchboardRepository, quest_viewing::QuestViewingRepository,
    },
    value_objects::{
        quest_adventurer_junction::{QuestAdventurerJunction, MAX_ADVENTURERS_PER_QUEST},
        quest_statuses::QuestStatuses,
    },
};

pub struct CrewSwitchboardUseCase<T1, T2>
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    crew_switchboard_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> CrewSwitchboardUseCase<T1, T2>
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(crew_switchboard_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            crew_switchboard_repository,
            quest_viewing_repository,
        }
    }

    pub async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurer_count = self
            .quest_viewing_repository
            .adventurer_counting_by_quest_id(quest_id)
            .await?;

        let quest_status_condition = quest.status == QuestStatuses::Open.to_string()
            || quest.status == QuestStatuses::Failed.to_string();

        let adventurers_count_condition = adventurer_count < MAX_ADVENTURERS_PER_QUEST;

        if !adventurers_count_condition {
            return Err(anyhow::anyhow!("Quest is full"));
        }

        if !quest_status_condition {
            return Err(anyhow::anyhow!("Quest is not started"));
        }

        self.crew_switchboard_repository
            .join(QuestAdventurerJunction {
                quest_id,
                adventurer_id,
            })
            .await?;

        Ok(())
    }
    pub async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let leave_condition = quest.status == QuestStatuses::Open.to_string()
            || quest.status == QuestStatuses::Failed.to_string();

        if !leave_condition {
            return Err(anyhow::anyhow!("Quest is not leaveable"));
        }

        self.crew_switchboard_repository
            .leave(QuestAdventurerJunction {
                quest_id,
                adventurer_id,
            })
            .await?;

        Ok(())
    }
}
