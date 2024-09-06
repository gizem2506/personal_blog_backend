// src/handlers/services.rs
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::ServicesData;
use serde_json::json;

pub async fn get_services_data(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        ServicesData,
        r#"
        SELECT id, title
        FROM services_data
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_services_data(pool: web::Data<PgPool>, item: web::Json<ServicesData>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO services_data (title)
        VALUES ($1)
        RETURNING id
        "#,
        item.title,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(json!({ "id": record.id })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_services_data(pool: web::Data<PgPool>, item: web::Json<ServicesData>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE services_data
        SET title = $1
        WHERE id = $2
        "#,
        item.title,
        item.id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_services_data(
    pool: web::Data<PgPool>,
    path: web::Path<i32>
) -> impl Responder {
    let id = path.into_inner(); 

    let result = sqlx::query!(
        r#"
        DELETE FROM services_data
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
