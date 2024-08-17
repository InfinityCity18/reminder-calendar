use anyhow::{Ok, Result};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

const BIND_SOCK_ADDR: &str = "localhost:2137"; // change to take from env or arg !!!!!

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(BIND_SOCK_ADDR).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

async fn send_reminders() -> {}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RemindersData {}
