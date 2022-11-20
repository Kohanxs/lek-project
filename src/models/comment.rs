use juniper::{GraphQLObject, Nullable};

use crate::schema::comments;
use crate::models::user::User;
use crate::models::question::{QuestionDB, Question};



#[derive(GraphQLObject, Debug, diesel::Associations, diesel::Identifiable, diesel::Queryable, Serialize, Deserialize)]
#[belongs_to(User, foreign_key="users_fk")]
#[belongs_to(QuestionDB, foreign_key="questions_fk")]
#[table_name = "comments"]
pub struct CommentDB {
    pub id: i32,
    pub content: String,
    pub suggested_answer: Option<i32>,
    pub users_fk: i32,
    pub questions_fk: i32,
    pub likes: i32
}

#[derive(GraphQLObject, Debug)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub suggested_answer: Option<i32>,
    pub user: User,
    pub likes: i32
}

impl From<(CommentDB, User)> for Comment {
    fn from((comment, user): (CommentDB, User)) -> Self {
        Comment {
            id: comment.id,
            content: comment.content,
            suggested_answer: comment.suggested_answer,
            user: user,
            likes: comment.likes
        }
    }
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

#[derive(Deserialize, juniper::GraphQLInputObject)]
pub struct DeleteComment {
    pub id: i32
}

#[derive(juniper::GraphQLInputObject)]
pub struct ModifyCommentForm {
    pub id: i32,
    pub content: Option<String>,
    pub suggested_answer: Nullable<i32>
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "comments"]
pub struct ModifyComment {
    pub id: i32,
    pub content: Option<String>,
    pub suggested_answer: Option<Option<i32>>
}

impl Into<ModifyComment> for ModifyCommentForm {
    fn into(self) -> ModifyComment {
        ModifyComment {
            // The `explicit` function transforms the `Nullable` into an
            // `Option<Option<T>>` as expected by the business logic layer.
            id: self.id,
            content: self.content,
            suggested_answer: self.suggested_answer.explicit()
        }
    }
}