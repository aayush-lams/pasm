use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use crate::{
    server::api::{
        amend,
        auth::{register, remove, update},
        backup, create, delete, find, health, list, users,
    },
    types::{db::PgDb, state::PasmState},
    utils::config,
};

pub mod api;
pub mod auth;
pub mod sql;

/// This function is the main entry point to server listener
/// It loads runtime variables, defines routes and starts listener and starts server
pub async fn run() {
    dotenv().ok();

    let database_url = config::database_url();

    let pool = PgPoolOptions::new()
        .max_connections(config::max_connections())
        .connect(&database_url)
        .await
        .expect("failed to connect to PostgreSQL");

    // Run idempotent schema migration on startup
    sqlx::raw_sql(include_str!("sql/migrations/001_init.sql"))
        .execute(&pool)
        .await
        .expect("failed to run database migrations");

    let state = PasmState {
        db: PgDb::new(pool),
        started_at: std::time::Instant::now(),
    };

    let protected_routes = Router::new()
        .route("/entries", get(list::call))
        .route("/entry", post(create::call))
        .route("/entry/amend", post(amend::call))
        .route("/entry/{name}", delete(delete::call).get(find::call))
        .route("/auth/update", post(update::call))
        .route("/auth/remove", delete(remove::call))
        .route("/auth/list", get(users::call))
        .route("/backup", get(backup::call))
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(state.clone(), auth::call)); // probable layers : cors, ratelimit, logging

    let public_routes = Router::new()
        .route("/auth", post(register::call))
        .route("/health", get(health::call))
        .with_state(state.clone());

    let app = public_routes.merge(protected_routes);
    let bind_addr = config::server_addr();
    let listener = TcpListener::bind(&bind_addr).await.unwrap();
    println!("Server listening on {bind_addr}");
    axum::serve(listener, app).await.unwrap();
}
