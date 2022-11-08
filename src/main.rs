use actixweb::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;
use services::get_all;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    // let db = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&std::env::var("DATABASE_URL").unwrap())
    //     .await
    //     .unwrap();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error connecting to database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} 