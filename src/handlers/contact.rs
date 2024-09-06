use crate::models::ContactForm;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

// Create (Save)
async fn save_to_database(contact_form: &ContactForm, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO contact_forms (name, email, subject, message) VALUES ($1, $2, $3, $4)",
        contact_form.name,
        contact_form.email,
        contact_form.subject,
        contact_form.message
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn get_all_from_database(pool: &PgPool) -> Result<Vec<ContactForm>, sqlx::Error> {
    let records = sqlx::query_as!(
        ContactForm,
        "SELECT id, name, email, subject, message FROM contact_forms"
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}
pub async fn submit_form(form: web::Json<ContactForm>, pool: web::Data<PgPool>) -> impl Responder {
    let form_data = form.into_inner();

    // Temel doğrulama
    if form_data.name.is_empty()
        || form_data.email.is_empty()
        || form_data.subject.is_empty()
        || form_data.message.is_empty()
    {
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Tüm alanlar gereklidir."
        }));
    }

    if !form_data.email.contains('@') {
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Geçersiz e-posta formatı."
        }));
    }

    if let Err(e) = save_to_database(&form_data, &pool).await {
        eprintln!("Error saving contact form to database: {}", e);
        return HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Mesajınızı kaydetme başarısız oldu. Lütfen daha sonra tekrar deneyin."
        }));
    }

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Mesajınız başarıyla kaydedildi. En kısa sürede sizinle iletişime geçeceğiz."
    }))
}

// Retrieve all contact forms
pub async fn get_all_contact_forms(pool: web::Data<PgPool>) -> impl Responder {
    match get_all_from_database(&pool).await {
        Ok(contact_forms) => HttpResponse::Ok().json(contact_forms),
        Err(e) => {
            eprintln!("Error retrieving contact forms: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to retrieve contact forms. Please try again later."
            }))
        }
    }
}
