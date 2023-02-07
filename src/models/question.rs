use crate::schema::questions;
use crate::models::category;
use juniper::{GraphQLObject, GraphQLInputObject};


#[derive(Debug, GraphQLObject)]
pub struct Question {
    pub id: i32,
    pub content: String,
    pub answers: Vec<String>,
    pub correct_answer: Option<i32>,
    pub categories: Vec<category::Category>,
}

#[derive(Debug, GraphQLInputObject)]
pub struct ModifyQuestionForm {
    pub context: String,
    pub answers: Vec<String>,
    pub correct_answer: Option<i32>,
}

pub struct ModifyQuestion {
    pub context: String,
    pub answer_1: String,
    pub answer_2: String,
    pub answer_3: String,
    pub answer_4: String,
    pub answer_5: String,
    pub correct_answer: Option<i32>,
}

#[derive(Debug, GraphQLObject, diesel::Queryable, diesel::Identifiable, serde::Serialize, serde::Deserialize)]
#[table_name="questions"]
pub struct QuestionDB {
    pub id: i32,
    pub content: String,
    pub answer_1: String,
    pub answer_2: String,
    pub answer_3: String,
    pub answer_4: String,
    pub answer_5: String,
    pub correct_answer: Option<i32>,
}



impl From<(QuestionDB, Vec<category::Category>)> for Question{
    fn from((q, categories): (QuestionDB, Vec<category::Category>)) -> Self {
        Question {
            id: q.id,
            content: q.content,
            answers: vec![
                q.answer_1,
                q.answer_2,
                q.answer_3,
                q.answer_4,
                q.answer_5
                ],
            correct_answer: q.correct_answer,
            categories: categories
        }
    }
}
