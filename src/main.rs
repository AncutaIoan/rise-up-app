mod state;
mod models;
mod handler;
mod error;
mod service;
mod repository;

use std::error::Error;
use actix_identity::IdentityMiddleware;
use actix_web::{middleware::Logger, App, HttpServer, web::Data, web};
use dotenv::dotenv;
use sqlx::PgPool;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_governor::governor::middleware::NoOpMiddleware;
use crate::handler::user_handler::{create_user, get_user_by_email, login};
use crate::state::app_state::AppState;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let app_state = create_app_state().await?;
    let governor_config = create_ip_governor();
    let server_address = "127.0.0.1:8080";

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(Logger::default())
            .wrap(Governor::new(&governor_config))
            .service(
                web::scope("/users")
                    .app_data(Data::new(app_state.user_state.clone()))
                    .service(create_user)
                    .service(get_user_by_email)
                    .service(login)
            )
    })
        .bind(server_address)?
        .run()
        .await?;

    Ok(())
}

async fn create_app_state() -> Result<Data<AppState>, Box<dyn Error>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    Ok(Data::new(AppState::new(pool)))
}

fn create_ip_governor() -> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware> {
    GovernorConfigBuilder::default()
        .seconds_per_request(5)
        .burst_size(10)
        .finish()
        .unwrap()
}