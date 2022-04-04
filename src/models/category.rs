

#[derive(Debug, diesel::Queryable, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}