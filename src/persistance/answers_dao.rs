use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswerDao {
    async fn create_answer(&self,, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answer(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswersDaoImpl { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        //Use the 'sqlx::types::Uuid::parse_str' method to parse the question_uuid field in 'Answer' struct into a 'Uuid' type
        //parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //If 'parse_str' returns an error, map the error (clue: use an iterator) to a 'DBError::InvalidUUID; error and early early from the function
        let uuis = todo!();

        //Make a database query to insert a new answer. Here is the SQL query:
        // Here is the SQL query:
        // ```
        // INSERT INTO answers ( question_uuid, content )
        // VALUES ( $1, $2 )
        // RETURNING *
        // ```
        //If executing the query results in an error, check to see if the error code matches 'postgres_error_codes::FOREIGN_KEY_VIOLATION'.
        //If so, early return the 'DBError::InvalidUUID' error. Otherwise, early return the 'DBError::Other' error.
        let record = todo!();
        //Populate the AnswersDetail fields using 'record'
        Ok(AnswerDetail {
            answer_uuid:: todo!(),
            question_uuid:: todo!(),
            content: todo!(),
            created_at:: todo!(),
        })
   }

   async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        //Use the 'sqlx::types::Uuid::parse_str' method to parse 'answer_uuid' into a 'Uuid' type.
        //parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //If 'parse_str' returns an error, map the error to a 'DBError::InvalidUUID' error and early return from this function.
        let uuid = todo!();

        //TODO: Make a database query to delete an answer given the answer_uuid. Here is the SQL query:
           // ```
        // DELETE FROM answers WHERE answer_uuid = $1
        // ```
        //If executing the query results in an error, map that error to a "'DBError::Other' error and early return from this function.
        let records = todo!();
        //Iterate over 'records' and map each record to an 'AnswerDetail' type
        let answers = todo!();

        Ok(answers)

   }

}