#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::migrate::MigrateDatabase;
use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::sqlite::{SqlitePoolOptions, Sqlite};

mod handlers;
mod models;
mod persistance;

use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(DB_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone()));
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool));

    let app_state = AppState {
        questions_dao,
        answers_dao,
    };

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
