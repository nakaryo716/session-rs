use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

// Session struct can save some kind of data
// The KEY was used to sort some kind of data that we want to save
const KEY: &str = "counter";
const KEY2: &str = "text";

// The data that insert into session-sotre need to impl Deserialize and Serialize
// This struct "Counter" associated with "KEY = "counter"
#[derive(Debug, Serialize, Deserialize, Default)]
struct Counter(usize);

// This struct "Data" associated with "KEY2 = "text"
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Data {
    text: String,
}

// Session struct provide some method
// We used .get() and .insert()
// eatch method take KEY to sort data

pub async fn session_count(session: Session) -> impl IntoResponse {
    let counter: Counter = session.get(KEY).await.unwrap().unwrap_or_default();

    session.insert(KEY, counter.0 + 1).await.unwrap();
    format!("Current count is: {}", counter.0)
}

pub async fn session_post_data(session: Session, Json(payload): Json<Data>) -> impl IntoResponse {
    let data: Data = session.get(KEY2).await.unwrap().unwrap_or_default();

    session.insert(KEY2, payload).await.unwrap();
    format!("The text you inputed previous is: {}", data.text)
}

pub async fn session_get_data(session: Session) -> impl IntoResponse {
    let data: Data = session.get(KEY2).await.unwrap().unwrap_or_default();

    format!("The text stored is: {}", data.text)
}
