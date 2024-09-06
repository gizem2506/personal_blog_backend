
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AboutData {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
}
