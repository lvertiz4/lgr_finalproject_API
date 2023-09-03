use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Question {
    //TODO: add a public 'title' field of type String to
    //TODO: add a public 'description' field of type String
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    //TODO: add a public 'question_uuid" field of type String
    //TODO: add a public 'title' field of type String
    //TODO: add a public 'description' field of type String
    //TODO: add a public 'created_at' field of type String
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

//TODO: create a struct called 'QuestionID'
// derive the following traits: Serialized, Deserialized
//add a public 'question_uuid' field of type String
//add a public 'content' field of type String

#[derive(Serialize, Deserialize)]
pub struct QuestionId {
    pub question_uuid: String,
}

//TODO: create a struct called 'Answer'
// derive the following traits: Serialized, Deserialized
// add a public 'question_uuid' field of type String
// add a public 'content' field of type String

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

//TODO: create a struct called 'AnswerDetail'
//derive the following traits: Serialized, Deserialize, Debug, Clone, PartialEq
// add a public 'answer_uuid' field of type String
// add a public 'question_uuid' field of type String
// add a public 'content' field of type String
// add a public 'create_at' field of type String

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

//TODO: create a struct called 'AnswerID'
// derive the following traits: Serialized, Deserialize
//add a public 'answer_id' field of type String
#[derive(Serialize, Deserialize)]
pub struct AnswerId {
    pub answer_uuid: String,
}

#[derive(Error, Debug)]
pub enum DBError { //We will use this error type inside the Direct access object (D-O-A)
    #[error("Invalid UUID provided: {0}")]
    InvalidUUID(String),
    #[error("Database error occurred: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

// source: https://www.postgresql.org/docs/current/errcodes-appendix.html
pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}