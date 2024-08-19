use anyhow::Result;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing::instrument;

const BIND_SOCK_ADDR: &str = "10.21.37.100:2137"; // change to take from env or arg !!!!!

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    info!("Tracing initialized");
    let path = "data.json";

    let app = Router::new()
        .route("/reminders", get(|| async { send_reminders(path).await }))
        .route(
            "/checkbox",
            post(|Json(payload): Json<CheckboxPostData>| async { checkbox(payload, path).await })
                .options(cors_shenanigans),
        )
        .route(
            "/add",
            post(|Json(payload): Json<ReminderAddData>| async {
                add_reminder(payload, path).await
            })
            .options(cors_shenanigans),
        )
        .route(
            "/delete",
            post(|Json(payload): Json<ReminderDeleteData>| async {
                delete_reminder(payload, path).await
            })
            .options(cors_shenanigans),
        );

    let listener = tokio::net::TcpListener::bind(BIND_SOCK_ADDR).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

#[instrument]
async fn add_reminder(payload: ReminderAddData, path: &str) -> impl IntoResponse {
    info!("ADD REMINDER");
    use std::io::BufWriter;
    let mut d = get_data_from_json_file(path).unwrap();

    let backup = std::fs::File::create(path.to_owned() + ".backup").unwrap();
    let backup_writer = BufWriter::new(backup);
    serde_json::to_writer(backup_writer, &d).unwrap();

    for month_to_add_index in payload.months {
        let month = &mut d.months_reminders[month_to_add_index as usize];
        let remd = Reminder {
            name: payload.name.clone(),
            day: payload.day as u8,
            checked: false,
            month: month_to_add_index as u8,
        };
        month.push(remd);
    }
    let file = std::fs::File::create(path).unwrap();
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, &d).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers
}

#[instrument]
async fn delete_reminder(payload: ReminderDeleteData, path: &str) -> impl IntoResponse {
    info!("GOT DELETE REMINDER : {:?}", &payload);
    use std::io::BufWriter;
    let mut d = get_data_from_json_file(path).unwrap();

    let backup = std::fs::File::create(path.to_owned() + ".backup").unwrap();
    let backup_writer = BufWriter::new(backup);

    serde_json::to_writer(backup_writer, &d).unwrap();

    for v in &mut d.months_reminders {
        v.retain(|r| {
            return !(r.name == payload.name);
        })
    }

    let file = std::fs::File::create(path).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &d).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers
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
    info!("GOT REMINDER CLIENT");
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    (headers, Json(get_data_from_json_file(path).unwrap()))
}
#[instrument]
async fn checkbox(payload: CheckboxPostData, path: &str) -> impl IntoResponse {
    use std::io::BufWriter;
    let mut d = get_data_from_json_file(path).unwrap();

    let backup = std::fs::File::create(path.to_owned() + ".backup").unwrap();
    let backup_writer = BufWriter::new(backup);
    serde_json::to_writer(backup_writer, &d).unwrap();

    for remd in &mut d.months_reminders[payload.reminder_month as usize] {
        info!("REMINDER : {:?}", remd);
        info!("PAYLOAD : {:?}", payload);
        if remd.name == payload.reminder_name {
            remd.checked = payload.checked;
        }
    }
    info!("d : {:?}", &d);

    let file = std::fs::File::create(path).unwrap();
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, &d).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReminderAddData {
    pub day: i32,
    pub name: String,
    pub months: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReminderDeleteData {
    pub name: String,
}
