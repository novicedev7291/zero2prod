use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool_data: web::Data<PgPool>) -> HttpResponse {
    log::info!("Saving new subscriber's details into database");
    match sqlx::query!(
        r#"insert into subscriptions(id, email, name, subscribed_at) values($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool_data.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            log::error!("Failed to save subscriber details : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
