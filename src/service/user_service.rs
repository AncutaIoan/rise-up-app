use crate::models::user::User;
use std::sync::Arc;
use crate::error::user_error::UserError;
use crate::repository::user_repository::UserRepository;

pub struct UserService {
    repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, user: &User) -> Result<User, UserError> {
        self.repo
            .create_user(user)
            .await
            .map_err(UserError::from)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<User, UserError> {
        let user_opt = self
            .repo
            .find_by_email(email)
            .await
            .map_err(UserError::from)?;

        user_opt.ok_or(UserError::NotFound)
    }
}
