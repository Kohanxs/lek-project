use juniper::GraphQLObject;

use crate::schema::comments;
use crate::models::user::User;
use crate::models::question::Question;



#[derive(GraphQLObject, Debug, diesel::Associations, diesel::Identifiable, diesel::Queryable, Serialize, Deserialize)]
#[belongs_to(User, foreign_key="users_fk")]
#[belongs_to(Question, foreign_key="questions_fk")]
#[table_name = "comments"]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub suggested_answer: Option<i32>,
    pub questions_fk: i32,
    pub users_fk: i32,
    pub likes: i32
}



#[derive(Debug, Deserialize, diesel::Insertable)]
#[table_name = "comments"]
pub struct InsertableComment {
    pub content: String,
    pub suggested_answer: Option<i32>,
    pub users_fk: i32,
    pub questions_fk: i32
}

#[derive(Deserialize, juniper::GraphQLInputObject)]
pub struct NewComment {
    pub content: String,
    pub question: i32,
    pub suggested_answer: Option<i32>
}

