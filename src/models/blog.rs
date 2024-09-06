
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BlogData {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub img: String,
    pub url: String,
}
