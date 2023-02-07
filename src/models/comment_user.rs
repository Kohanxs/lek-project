use crate::schema::comment_user;

#[derive(Debug, diesel::Associations, diesel::Identifiable, diesel::Queryable, Serialize, Deserialize)]
#[primary_key("comment_fk", "user_fk")]
#[table_name = "comment_user"]
pub struct CommentUser {
    pub comment_fk: i32,
    pub user_fk: i32
}