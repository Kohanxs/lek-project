

#[derive(Debug, diesel::Queryable, Serialize, Deserialize, juniper::GraphQLObject)]
pub struct Category {
    pub id: i32,
    pub name: String,
}