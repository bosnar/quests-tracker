use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, post},
    Extension, Json, Router,
};

use crate::{
    application::usecases::crew_switchboard::CrewSwitchboardUseCase,
    domain::repositories::{
        crew_switchboard::CrewSwitchboardRepository, quest_viewing::QuestViewingRepository,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            crew_switchboard::CrewSwitchboardPostgres, quest_viewing::QuestViewingPostgres,
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_switchboard_repository = CrewSwitchboardPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let crew_switchboard_usecase = CrewSwitchboardUseCase::new(
        Arc::new(crew_switchboard_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/join/:quest_id", post(join))
        .route("/leave/:quest_id", delete(leave))
        .with_state(Arc::new(crew_switchboard_usecase))
}

pub async fn join<T1, T2>(
    State(crew_switchboard_usecase): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>, // รับ quest_id จาก path ชื่อต้องเหมือนกัน
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn leave<T1, T2>(
    State(crew_switchboard_usecase): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
