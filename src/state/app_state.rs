use std::sync::Arc;
use sqlx::PgPool;
use crate::repository::repository::PostgresRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::user_service::UserService;

pub struct AppState {
    pub user_state: UserState,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let user_repo = Arc::new(PostgresRepository::new(pool.clone())) as Arc<dyn UserRepository + Send + Sync>;

        let user_service = Arc::new(UserService::new(user_repo));

        AppState {
            user_state: UserState { user_service }
        }
    }
}

#[derive(Clone)]
pub struct UserState {
    pub user_service: Arc<UserService>
}