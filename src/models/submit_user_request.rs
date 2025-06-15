use crate::models::user::{User, UserRole};
use bcrypt::hash;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitUserRequest {
    pub email: String,
    pub password: String
}
impl SubmitUserRequest {
    pub fn to_user(&self) -> User {
        User {
            id: Uuid::new_v4(),
            email: self.email.clone(),
            hashed_password: hash(&self.password, 10).unwrap(),
            role: UserRole::User,
            created_at: Utc::now(),
        }
    }
}