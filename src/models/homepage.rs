#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct HomePageData {
    pub id: i32,
    pub img: String,
    pub title: String,
    pub subtitle: String,
    pub email: String,
}
