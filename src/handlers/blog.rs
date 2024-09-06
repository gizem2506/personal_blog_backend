use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::BlogData; 
use serde_json::json; 




pub async fn get_blog_data(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(
        BlogData,
        r#"
        SELECT id, img, url,title, subtitle
        FROM blog_data
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}



pub async fn create_blog_data(pool: web::Data<PgPool>, item: web::Json<BlogData>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO blog_data ( img, url,title, subtitle)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        item.img,
        item.title,
        item.subtitle,
        item.url
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(json!({ "id": record.id })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_blog_data(pool: web::Data<PgPool>, item: web::Json<BlogData>) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE blog_data
        SET img = $1, title = $2, subtitle = $3, url = $4
        WHERE id = $5
        "#,
        item.img,
        item.title,
        item.subtitle,
        item.url,
        item.id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_blog_data(
    pool: web::Data<PgPool>,
    path: web::Path<i32>
) -> impl Responder {
    let id = path.into_inner(); 

    let result = sqlx::query!(
        r#"
        DELETE FROM blog_data
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
