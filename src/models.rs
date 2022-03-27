
use diesel;
use serde_derive::{Serialize, Deserialize};
use crate::schema::questions;


#[derive(Debug, diesel::Queryable, Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub text: String,
    pub answer_1: String,
    pub answer_2: String,
    pub answer_3: String,
    pub answer_4: String,
    pub answer_5: String,
    pub correct_answer: Option<i32>
}

#[derive(Deserialize, diesel::Insertable)]
#[table_name = "questions"]
pub struct InsertableQuestion {
    pub id: i32,
    pub text: String,
}

