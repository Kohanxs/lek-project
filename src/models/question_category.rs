use crate::models::category::{Category};
use crate::models::question::{QuestionDB};
use crate::schema::question_category;

#[derive(Debug, diesel::Queryable, diesel::Identifiable, diesel::Associations, serde::Serialize, serde::Deserialize)]
#[primary_key("question_fk", "category_fk")]
#[table_name = "question_category"]
#[belongs_to(QuestionDB, foreign_key = "question_fk")]
#[belongs_to(Category, foreign_key = "category_fk")]
pub struct QuestionCategory {
    question_fk: i32,
    category_fk: i32
}
