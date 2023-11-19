use axum::{
    routing::get,
    Router, 
    extract::FromRef
};
use sea_orm::DatabaseConnection;


#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    
        Router::new()
        .route("/", get(hello_world))
        .with_state(app_state)
}

pub async fn hello_world() -> String {
    "Hello world!".to_owned()
}