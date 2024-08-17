use anyhow::Result;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tracing::debug;
use tracing::instrument;

const BIND_SOCK_ADDR: &str = "localhost:2137"; // change to take from env or arg !!!!!

#[tokio::main]
async fn main() -> Result<()> {
    let path = "data.json";

    let app = Router::new()
        .route("/reminders", get(|| async { send_reminders(path).await }))
        .route(
            "/checkbox",
            post(|Json(payload): Json<CheckboxPostData>| async { checkbox(payload, path).await })
                .options(cors_shenanigans),
        );

    let listener = tokio::net::TcpListener::bind(BIND_SOCK_ADDR).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

#[instrument]
async fn cors_shenanigans() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "*".parse().unwrap());
    headers
}
#[instrument]
async fn send_reminders(path: &str) -> impl IntoResponse {
    debug!("GOT REMINDER CLIENT");
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    (headers, Json(get_data_from_json_file(path).unwrap()))
}
#[instrument]
async fn checkbox(payload: CheckboxPostData, path: &str) {
    debug!("GOT checkbox CLIENT");
    use std::io::BufWriter;
    let mut d = get_data_from_json_file(path).unwrap();
    for remd in &mut d.months_reminders[payload.reminder_month as usize] {
        debug!("REMINDER : {:?}", remd);
        debug!("PAYLOAD : {:?}", payload);
        if remd.name == payload.reminder_name {
            remd.checked = payload.checked;
        }
    }

    let file = std::fs::File::create(path).unwrap();
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, &d).unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RemindersData {
    pub months_names: Vec<String>,
    pub months_reminders: Vec<Vec<Reminder>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Reminder {
    pub name: String,
    pub day: u8,
    pub checked: bool,
    pub month: u8,
}

fn get_data_from_json_file(path: &str) -> Result<RemindersData> {
    let f = std::fs::File::open(path)?;
    let r = std::io::BufReader::new(f);

    let d: RemindersData = serde_json::from_reader(r)?;
    Ok(d)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct CheckboxPostData {
    checked: bool,
    reminder_name: String,
    reminder_month: u8,
}
