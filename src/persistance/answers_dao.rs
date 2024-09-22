use async_trait::async_trait;
use sqlx::query;
use sqlx::sqlite::SqlitePool;

use crate::models::{sqlite_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: SqlitePool,
}

impl AnswersDaoImpl {
    pub fn new(db: SqlitePool) -> Self {
        AnswersDaoImpl { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        /* let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!(
                "Could not parse answer UUID: {}",
                answer.question_uuid
            ))
        })?; */

        let uuid: i64 = answer.question_uuid.parse().map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse answer UUID: {}", answer.question_uuid))
        })?;
        
        let record = query!(
            r#"
                INSERT INTO answers ( question_uuid, content )
                VALUES ( ?, ? )
                RETURNING *
            "#,
            uuid,
            answer.content
        )
            .fetch_one(&self.db)
            .await
            .map_err(|e: sqlx::Error| match e {
                sqlx::Error::Database(e) => {
                    if let Some(code) = e.code() {
                        if code.eq(sqlite_error_codes::FOREIGN_KEY_VIOLATION) {
                            return DBError::InvalidUUID(format!(
                                "Invalid question UUID: {}",
                                answer.question_uuid
                            ));
                        }
                    }
                    DBError::Other(Box::new(e))
                }
                e => DBError::Other(Box::new(e)),
            })?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.unwrap().to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        /*let uuid = sqlx::types::Uuid::parse_str(&answer_uuid).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse question UUID: {}", answer_uuid))
        })?;*/

        let uuid: i64 = answer_uuid.parse().map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse question UUID: {}", answer_uuid))
        })?;

        query!("DELETE FROM answers WHERE answer_uuid = ?", uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
         let uuid: i64 = question_uuid.parse().map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse question UUID: {}", question_uuid))
        })?;
        
        let records = query!("SELECT * FROM answers WHERE question_uuid = ?", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = records
            .iter()
            .map(|r| AnswerDetail {
                answer_uuid: r.answer_uuid.to_string(),
                question_uuid: r.question_uuid.to_string(),
                content: r.content.clone(),
                created_at: r.created_at.to_string(),
            })
            .collect();

        Ok(answers)
    }
}
