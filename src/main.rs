use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to create pool");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() 
            .allow_any_method()
            .allow_any_header();
        
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .route("/homepage", web::get().to(handlers::homepage::get_homepage_data))
                    .route("/homepage", web::post().to(handlers::homepage::create_homepage_data))
                    .route("/homepage", web::put().to(handlers::homepage::update_homepage_data))
                    .route("/homepage/{id}", web::delete().to(handlers::homepage::delete_homepage_data))
                    // About Data Routes
                    .route("/about-data", web::get().to(handlers::about::get_about_data))
                    .route("/about-data", web::post().to(handlers::about::create_about_data))
                    .route("/about-data", web::put().to(handlers::about::update_about_data))
                    .route("/about-data/{id}", web::delete().to(handlers::about::delete_about_data))
                    // Skills Data Routes
                    .route("/skills-data", web::get().to(handlers::skills::get_skills_data))
                    .route("/skills-data", web::post().to(handlers::skills::create_skills_data))
                    .route("/skills-data", web::put().to(handlers::skills::update_skills_data))
                    .route("/skills-data/{id}", web::delete().to(handlers::skills::delete_skills_data))
                    // Services Data Routes
                    .route("/services-data", web::get().to(handlers::services::get_services_data))
                    .route("/services-data", web::post().to(handlers::services::create_services_data))
                    .route("/services-data", web::put().to(handlers::services::update_services_data))
                    .route("/services-data/{id}", web::delete().to(handlers::services::delete_services_data))
                    // Mail
                    .route("/contact", web::post().to(handlers::contact::submit_form))     
                    .route("/getcontact", web::get().to(handlers::contact::get_all_contact_forms))

                    .route("/blog", web::get().to(handlers::blog::get_blog_data))
                    .route("/blog", web::post().to(handlers::blog::create_blog_data))
                    .route("/blog", web::put().to(handlers::blog::update_blog_data))
                    .route("/blog/{id}", web::delete().to(handlers::blog::delete_blog_data))
      
    
                      )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
