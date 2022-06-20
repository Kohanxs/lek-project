use crate::schema::questions;
use crate::models::category;
use juniper::{GraphQLObject};

use super::comment::Comment;

#[derive(Debug, GraphQLObject, diesel::Queryable, diesel::Identifiable, diesel::Associations, Serialize, Deserialize)]
#[belongs_to(category::Category, foreign_key="category_fk")]
pub struct Question {
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


#[derive(Deserialize, juniper::GraphQLInputObject)]
pub struct QueryQuestion {
    pub id: Option<i32>,
    pub category_id: Option<i32>,
    pub suggested_answer: Option<i32>
}