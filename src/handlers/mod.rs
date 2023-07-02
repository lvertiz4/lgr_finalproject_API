use rocket::{serde::json::Json};

use crate::models::*;

// ---CRUD for Questions---

#[post("/question", data = "<question>")]
pub async fn create_question(question: Json<Question>) -> Json<QuestionDetail> {
    Json ( //this is 'rocket::serde::json::Json => Rocket's JSON guard to easily consume and return a JSON object, in this case the QuestionDetail Json object
        //To respond with serialized JSON data, return a Json<T> type, where T implements Serialize from serde; The QuestionDetail struct does that in line 11 of models.rs
        //All the Structs can deserialize into a Json object, or deserialize from a Json object back into a Rust struct
        // To deserialize request body data as JSON , add a data route argument with a target type of Json<T>, i.e. #[post("/question", data = "<user>")]
        QuestionDetail {
            question_uuid: "question_uuid".to_owned(),
            title: "title".to_owned(),
            description: "description".to_owned(),
            created_at: "created_at".to_owned()
        }
    )
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    Json (//This Json object contains a vector of QuestionDetail instances
        vec![QuestionDetail { 
            question_uuid: "question_uuid".to_owned(),
            title: "title".to_owned(),
            description: "description".to_owned(),
            created_at: "created_at".to_owned()
        }]
    )
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(question_uuid: Json<QuestionId>) {
    //nothing is here because the Delete path at the Json object of ObjectID, which has a question_uuid. When this object is deleted, the path doesn't return that object.
}

// ---CRUD for Answers---

//TODO: Create a POST route to /answer which accepts an 'Answer' and returns an 'AnswerDetail' as JSON.
// The handler function should be called 'create_answer'
// Hint: this function should look very similar to the create_question function above

#[post("/answer", data = "<answer>")]
pub async fn create_answer(answer: Json<Answer>) -> Json<AnswerDetail> {
   Json (
    AnswerDetail {
        answer_uuid: "answer_uuid".to_owned(),
        question_uuid: "question_uuid".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned()
    }
)
}

//TODO: Create a GET route to /answers which accepts an 'QuestionID' and returns a vector of 'AnswerDetail' as JSON
//The handler function should be called 'read_answer'
// hint: this function should look very similiar to the read_questions function above

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(question_uuid: Json<QuestionId>) -> Json<Vec<AnswerDetail>> {
    Json (
        vec![
            AnswerDetail {
                answer_uuid: "answer_uuid".to_owned(),
                question_uuid: "question_uuid".to_owned(),
                content: "content".to_owned(),
                created_at: "created_at".to_owned()
            }
        ]
        )
}

//TODO: Create a DELETE route to /answer which accepts an 'AnswerID' and does not return anything
// The handler function should be called 'delete_answer'
// hint: this function should look very similar to the delete_questions function above

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(answer_uuid: Json<AnswerId>) {
    
}