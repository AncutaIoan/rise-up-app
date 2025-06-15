use async_trait::async_trait;
use sqlx::Error;
use crate::models::user::User;
use crate::repository::repository::PostgresRepository;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User) -> Result<User, Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
}

#[async_trait]
impl UserRepository for PostgresRepository {
    async fn create_user(&self, user: &User) -> Result<User, Error> {
        let query = r#"
                            INSERT INTO users (id, email, hashed_password, role, created_at)
                            VALUES ($1, $2, $3, $4, $5)
                            RETURNING id, email, hashed_password, role, created_at
                        "#;

        match sqlx::query_as::<_, User>(query)
            .bind(&user.id)
            .bind(&user.email)
            .bind(&user.hashed_password)
            .bind(&user.role)
            .bind(&user.created_at)
            .fetch_one(&self.pool)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error inserting user: {}", e);
                Err(e)
            }
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let query = "SELECT id, email, hashed_password, role, created_at FROM users WHERE email = $1";

        match sqlx::query_as::<_, User>(query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                eprintln!("Error querying user by email: {}", e);
                Err(e)
            }
        }
    }
}
