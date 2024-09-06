// src/handlers/skills.rs
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::SkillsData;
use serde_json::json;

pub async fn get_skills_data(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        SkillsData,
        r#"
        SELECT id, title
        FROM skills_data
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_skills_data(pool: web::Data<PgPool>, item: web::Json<SkillsData>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO skills_data (title)
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

pub async fn update_skills_data(pool: web::Data<PgPool>, item: web::Json<SkillsData>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE skills_data
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

pub async fn delete_skills_data(
    pool: web::Data<PgPool>,
    path: web::Path<i32>
) -> impl Responder {
    let id = path.into_inner(); 

    let result = sqlx::query!(
        r#"
        DELETE FROM skills_data
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
