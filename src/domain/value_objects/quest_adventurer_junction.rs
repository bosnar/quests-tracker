use crate::{
    domain::entities::{adventurers::AdventurerEntity, quests::QuestEntity},
    infrastructure::postgres::schema::quest_adventurer_junction,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub const MAX_ADVENTURERS_PER_QUEST: i64 = 4;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Associations)]
#[diesel(belongs_to(AdventurerEntity, foreign_key = adventurer_id))]
#[diesel(belongs_to(QuestEntity, foreign_key = quest_id))]
#[diesel(table_name = quest_adventurer_junction)]
pub struct QuestAdventurerJunction {
    pub quest_id: i32,
    pub adventurer_id: i32,
}
