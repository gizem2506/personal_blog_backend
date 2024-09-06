use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use serde_json::json; 
use crate::models::HomePageData; 



pub async fn get_homepage_data(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        HomePageData,
        r#"
        SELECT id, img, title, subtitle, email
        FROM homepage_data
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}



pub async fn create_homepage_data(pool: web::Data<PgPool>, item: web::Json<HomePageData>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO homepage_data (img, title, subtitle, email)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        item.img,
        item.title,
        item.subtitle,
        item.email
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(json!({ "id": record.id })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_homepage_data(pool: web::Data<PgPool>, item: web::Json<HomePageData>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE homepage_data
        SET img = $1, title = $2, subtitle = $3, email = $4
        WHERE id = $5
        "#,
        item.img,
        item.title,
        item.subtitle,
        item.email,
        item.id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_homepage_data(
    pool: web::Data<PgPool>,
    path: web::Path<i32>
) -> impl Responder {
    let id = path.into_inner(); // Extract the id from the Path

    let result = sqlx::query!(
        r#"
        DELETE FROM homepage_data
        WHERE id = $1
        "#,
        id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
