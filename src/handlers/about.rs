// src/handlers/about.rs
use crate::models::AboutData;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

pub async fn get_about_data(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        AboutData,
        r#"
        SELECT id, title, subtitle
        FROM about_data
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_about_data(
    pool: web::Data<PgPool>,
    item: web::Json<AboutData>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO about_data (title, subtitle)
        VALUES ($1, $2)
        RETURNING id
        "#,
        item.title,
        item.subtitle
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(json!({ "id": record.id })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_about_data(
    pool: web::Data<PgPool>,
    item: web::Json<AboutData>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE about_data
        SET title = $1, subtitle = $2
        WHERE id = $3
        "#,
        item.title,
        item.subtitle,
        item.id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_about_data(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner(); // Extract the id from the Path

    let result = sqlx::query!(
        r#"
        DELETE FROM about_data
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
