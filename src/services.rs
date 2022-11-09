use actix_web::{get, post, web::{Json, Path}, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, FromRow)]
struct Article {
    id: i32,
    title: String,
    content: String,
    create_by: i32,
}

#[derive(Deserialize)]
pub struct CreateArticleBody {
    title: String,
    content: String,
}

#[get("/users")]
pub async fn fetch_users() -> impl Responder {
    "GET /users".to_string()
}

#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(path: Path<i32>) -> impl Responder {
  let id: i32 = path.into_inner();  
  format!("GET /users/{}/articles")
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(path: Path<i32>, body: Json<CreateArticleBody>) -> impl Responder {
  let id: i32 = path.into_inner();
  format!("POST /users/{}/articles")
}