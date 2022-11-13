use crate::schema::questions;
use crate::models::category;
use juniper::{GraphQLObject};


#[derive(Debug, GraphQLObject)]
pub struct Question {
    pub id: i32,
    pub content: String,
    pub answers: Vec<String>,
    pub correct_answer: Option<i32>,
    pub category: Option<category::Category>,
}

#[derive(Debug, GraphQLObject, diesel::Queryable, diesel::Identifiable, diesel::Associations, serde::Serialize, serde::Deserialize)]
#[belongs_to(category::Category, foreign_key="category_fk")]
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
    pub category_fk: Option<i32>,
}



impl From<(QuestionDB, Option<category::Category>)> for Question{
    fn from((q, category): (QuestionDB, Option<category::Category>)) -> Self {
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
            category: category
        }
    }
}
