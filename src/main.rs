use axum::routing::post;
use axum::{routing::get, Router};
use session::{session_count, session_get_data, session_post_data};
use time::Duration;
use tower_cookies::Key;
use tower_sessions::{MemoryStore, SessionManagerLayer};

mod session;

// tower_sessions using tower-cookie crate
// This code trying session manage by using tower-sessions and axum crate

#[tokio::main]
async fn main() {
    let key = Key::generate();

    // create session store
    // In this case, I used InMemory
    // We can use Mysql, Postgres, sqlite, Mongodb etc...
    let session_store = MemoryStore::default();

    // create session_layer
    // SessionManagerLayer is "Session struct" that costumed
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        // decide session time
        .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::seconds(30)))
        .with_signed(key);

    // Routing
    let app = Router::new()
        .route("/", get(session_count))
        .route("/text", post(session_post_data).get(session_get_data))
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
