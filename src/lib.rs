use sea_orm::Database;
pub mod routes;
// pub mod database;
// pub mod controllers;
// pub mod services;

pub async fn run(db_uri: &str){
    let db = Database::connect(db_uri).await.unwrap();
    let app = routes::create_routes(db).await;

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}