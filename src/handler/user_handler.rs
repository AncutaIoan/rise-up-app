use crate::models::user::User;

use crate::state::app_state::UserState;
use actix_web::web::{Data, Json};
use actix_web::{post, get, Result};
use crate::error::user_error::UserError;
use crate::models::login_request::LoginRequest;
use crate::models::submit_user_request::SubmitUserRequest;

#[post("")]
pub async fn create_user(
    user_state: Data<UserState>,
    Json(user_request): Json<SubmitUserRequest>,
) -> Result<Json<User>, UserError> {
    Ok(Json(user_state.user_service.create_user(&user_request.to_user()).await?))
}

#[get("/{email}")]
pub async fn get_user_by_email(
    user_state: Data<UserState>,
    email: actix_web::web::Path<String>,
) -> Result<Json<User>, UserError> {
    Ok(Json(user_state.user_service.find_by_email(&email).await?))
}

#[post("/login")]
pub async fn login(
    user_state: Data<UserState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<User>, UserError> {
    Ok(Json(user_state.user_service.login_user(&login_request).await?))
}