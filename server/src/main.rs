use anyhow::Result;
use axum::routing::{get, post};
use axum::{Json, Router};
use log::{debug, info};
use serde::{Deserialize, Serialize};

const BIND_SOCK_ADDR: &str = "localhost:2137"; // change to take from env or arg !!!!!

#[tokio::main]
async fn main() -> Result<()> {
    let path = "data.json";

    let app = Router::new()
        .route(
            "/reminders",
            get(|| async {
                info!("GOT A CLIENT");
                send_reminders(path).await
            }),
        )
        .route(
            "/checkbox",
            post(|Json(payload): Json<CheckboxPostData>| async { checkbox(payload, path).await }),
        );

    let listener = tokio::net::TcpListener::bind(BIND_SOCK_ADDR).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

async fn send_reminders(path: &str) -> Json<RemindersData> {
    debug!("GOT REMINDER REQUEST");
    Json(get_data_from_json_file(path).unwrap())
}

async fn checkbox(payload: CheckboxPostData, path: &str) {
    use std::io::BufWriter;
    let mut d = get_data_from_json_file(path).unwrap();
    for remd in &mut d.months_reminders[payload.reminder_month as usize] {
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
