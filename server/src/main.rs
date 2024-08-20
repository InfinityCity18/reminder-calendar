use anyhow::Result;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing::instrument;

const BIND_SOCK_ADDR: &str = "0.0.0.0:12137";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    info!("Tracing initialized");
    let path = "/reminder-calendar/server/data.json";

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
        )
        .route(
            "/uncheck",
            post(|Json(payload): Json<UncheckData>| async {
                uncheck_reminders(payload, path).await
            })
            .options(cors_shenanigans),
        )
        .route(
            "/notification",
            get(|| async { send_notification_info(path).await }),
        );

    let listener = tokio::net::TcpListener::bind(BIND_SOCK_ADDR).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

#[instrument]
async fn add_reminder(payload: ReminderAddData, path: &str) -> impl IntoResponse {
    info!("add reminder");
    use std::io::BufWriter;
    let mut d = get_data_from_json_file(path).unwrap();

    let backup = std::fs::File::create(path.to_owned() + ".backup").unwrap();
    let backup_writer = BufWriter::new(backup);
    serde_json::to_writer_pretty(backup_writer, &d).unwrap();

    for month_to_add_index in payload.months {
        let month = &mut d.months_reminders[month_to_add_index as usize];
        let remd = Reminder {
            name: payload.name.clone(),
            day: payload.day as u8,
            checked: false,
            month: month_to_add_index as u8,
            deadline_remind: payload.deadline_remind,
        };
        month.push(remd);
    }
    let file = std::fs::File::create(path).unwrap();
    let writer = BufWriter::new(file);

    info!("JSON BEFORE SAVING : {:?}", &d);
    serde_json::to_writer_pretty(writer, &d).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers
}

#[instrument]
async fn send_notification_info(path: &str) -> impl IntoResponse {
    use chrono::Local;

    info!("CLIENT WANTS NOTIFICATION INFO");

    let mut incoming = Vec::new();

    let d = get_data_from_json_file(path).unwrap();
    let now = Local::now();

    for v in d.months_reminders {
        for r in v {
            if !r.checked {
                let (day, month) =
                    if let None = (r.day as u32).checked_sub(r.deadline_remind as u32) {
                        let diff = r.day as i32 - r.deadline_remind as i32;
                        let day = get_days_from_month(now.year(), (r.month + 1) as u32 - 1);
                        let day = day as i32 + diff;
                        (day, r.month)
                    } else {
                        (r.day as i32 - r.deadline_remind as i32, r.month + 1)
                    };
                info!("DAY : {}, MONTH : {}", day, month);

                if now.day() == day as u32 && now.month() == month as u32 {
                    incoming.push(r.clone());
                }
            }
        }
    }

    let mut response = String::new();
    for r in incoming {
        response += &(format!(
            "{} : {} {}, ",
            r.name, d.months_names[r.month as usize], r.day
        ));
    }
    info!("RESPONSE : {:?}", response);
    response
}

async fn uncheck_reminders(_payload: UncheckData, path: &str) -> impl IntoResponse {
    info!("UNCHECKING REMINDERS");
    use std::io::BufWriter;
    let mut d = get_data_from_json_file(path).unwrap();

    let backup = std::fs::File::create(path.to_owned() + ".backup").unwrap();
    let backup_writer = BufWriter::new(backup);
    serde_json::to_writer_pretty(backup_writer, &d).unwrap();

    for v in &mut d.months_reminders {
        for r in v {
            r.checked = false;
        }
    }

    let file = std::fs::File::create(path).unwrap();
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &d).unwrap();

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

    serde_json::to_writer_pretty(backup_writer, &d).unwrap();

    for v in &mut d.months_reminders {
        v.retain(|r| {
            return !(r.name == payload.name);
        })
    }

    let file = std::fs::File::create(path).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &d).unwrap();

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
    serde_json::to_writer_pretty(backup_writer, &d).unwrap();

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

    serde_json::to_writer_pretty(writer, &d).unwrap();

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
    pub deadline_remind: i32,
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
    pub deadline_remind: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReminderDeleteData {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UncheckData {
    pub uncheck: bool,
}

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
    chrono::NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .expect("conversion failed")
    .signed_duration_since(
        chrono::NaiveDate::from_ymd_opt(year, month, 1).expect("conversion failed"),
    )
    .num_days()
}
