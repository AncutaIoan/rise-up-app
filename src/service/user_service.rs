use crate::models::user::User;
use std::sync::Arc;
use bcrypt::verify;
use crate::error::user_error::UserError;
use crate::models::login_request::LoginRequest;
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

    pub async fn login_user(&self, login_request: &LoginRequest) -> Result<User, UserError> {
        let user = self.find_by_email(&login_request.email).await?;

        let valid = verify(&login_request.password, &user.hashed_password)
            .map_err(|e| UserError::InternalError(format!("Password verify error: {}", e)))?;

        if !valid {
            return Err(UserError::InternalError("Verify password".to_string()));
        }

        Ok(user)
    }
}
