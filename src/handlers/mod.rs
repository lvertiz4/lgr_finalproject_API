use rocket::{serde::json::Json};

use crate::models::*;

// ---CRUD for Questions---

#[post("/question", data = "<question>")]
pub async fn create_question(question: Json<Question>,) -> Json<QuestionDetail> {
    todo!()
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    todo!()
}

#[delete("/questions", data = "<question_uuid>")]
pub async fn delete_questions(question_uuid: Json<QuestionID>) {
    todo!()
}

// ---CRUD for Answers---

//TODO: Create a POST route to /answer which accepts an 'Answer' and returns an 'AnswerDetail' as JSON.
// The handler function should be called 'create_answer'
// Hint: this function should look very similar to the create_question function above

#[post("/answer", data = "<answer>")]
pub async fn create_answer(answer: Json<Answer>) -> Json<AnswerDetail> {
    todo!()
}

//TODO: Create a GET route to /answers which accepts an 'QuestionID' and returns a vector of 'AnswerDetail' as JSON
//The handler function should be called 'read_answer'
// hint: this function should look very similiar to the read_questions function above

#[get("/answers", data = "<question_id>"]
pub async fn read_answer(question_id: Json<QuestionID>) -> Json<Vec<AnswerDetail>> {
    todo!()
}

//TODO: Create a DELETE route to /answer which accepts an 'AnswerID' and does not return anything
// The handler function should be called 'delete_answer'
// hint: this function should look very similar to the delete_questions function above

#[delete("/answer", data = "<answer_id>")]
pub async fn delete_answer(answer_id: Json<AnswerID>) {
    todo!()
}