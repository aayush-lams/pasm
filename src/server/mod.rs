use std::{env, path::Path, sync::Arc, thread, time::Duration};

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use dotenv::dotenv;
use sled::Db;
use tokio::net::TcpListener;

use crate::{
    server::api::{amend, create, delete, find, list},
    types::PasmState,
};

pub mod api;
pub mod auth;

/// This function is the main entry point to server listener
/// It loads runtime variables, defines routes and starts listener and starts server
pub async fn run() {
    dotenv().ok();
    let api_key = match env::var("API_KEY") {
        Ok(k) => k,
        Err(err) => {
            println!("could not find api key : {err:?}");
            thread::sleep(Duration::from_secs(3));
            return;
        }
    };
    let shared_api_key = Arc::new(api_key);

    let encryption_key = match env::var("ENCRYPTION_KEY") {
        Ok(k) => k,
        Err(err) => {
            println!("could not find api key : {err:?}");
            thread::sleep(Duration::from_secs(3));
            return;
        }
    };

    let shared_encr_key = Arc::new(encryption_key);

    let home_dir = match env::var("HOME") {
        Ok(home) => home,
        Err(err) => {
            println!("could not find api key : {err:?}");
            thread::sleep(Duration::from_secs(3));
            return;
        }
    };

    let filepath = Path::new(&home_dir)
        .join(".config")
        .join("path")
        .join("database");

    let db: Db = sled::open(filepath).expect(&"failed to open database!");

    let state = PasmState {
        db,
        api_key: shared_api_key,
        encr_key: shared_encr_key,
    };

    let protected_routes = Router::new()
        .route("/entries", get(list::call))
        .route("/entry", post(create::call))
        .route("/entry/amend", post(amend::call))
        .route("/entry/{name}", delete(delete::call).get(find::call))
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(state.clone(), auth::call));

    let public_routes = Router::new();

    let app = public_routes.merge(protected_routes);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening at : {:#?}", &listener);
    axum::serve(listener, app).await.unwrap();
}
