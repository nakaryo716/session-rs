# Repository to trying tower-session crate
This repository is axum server that implmented tower-session midleware  

## Detail of this crate
tower-session crate (https://crates.io/crates/tower-sessions) provides sessions, key-value pairs associated with a site visitor, as a tower middleware.  

## Usage (Crate over view)
### First step: *Create Session-Store*
Create session store.  
In this case, I used Memory.
We can use other way. Postgres, Mysql, Sqlite etc...  
session_store need to implment ```SessionStore``` Trait.
```rust
let session_store = MemoryStore::default();
```
The next step need a struct that implmented ```SessionStore``` Trait by this call.
```rust
pub struct SessionManagerLayer<Store: SessionStore, C: CookieController = PlaintextCookie> { /* private fields */ }
```

### Second step: *build session-layer*
build SessionManagerLayer by using session-store.  
There are a lot of method that implmented for SessionManagerLayer.  
These method are some settings.
```rust
let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false)
    // decide session time
    .with_expiry(tower_sessions::Expiry::OnInactivity(Duration::seconds(30)))
    .with_signed(key);
```

### Third step: *Share session-layer*
Axum provide ```.layer()``` method.  
By using this method, we can share session-layer other handler(dont have to wrap ```Arc```)
```rust
Router::new()
    .route("/", get(handler))
    .layer(session_layer)
```
The session_store is wraped by ```Arc``` in this crate library.
```rust
pub struct SessionManagerLayer<Store: SessionStore, C: CookieController = PlaintextCookie> {
    session_store: Arc<Store>,
    session_config: SessionConfig<'static>,
    cookie_controller: C,
}
```

### Fourth step: *Using session-layer*
Thank for ```.layer()```, we can use session-layer.  
Session struct provide a lot of method. See document.  
We dont have to think about cookie and session manage deeply!!
```rust
#[derive(Debug, Serialize, Deserialize, Default)]
struct Counter(usize);

pub async fn session_count(session: Session) -> impl IntoResponse {
    let counter: Counter = session.get(KEY).await.unwrap().unwrap_or_default();

    session.insert(KEY, counter.0 + 1).await.unwrap();
    format!("Current count is: {}", counter.0)
}
```
