
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ContactForm {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}


