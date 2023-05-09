use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(name = "Saving subscriber detail in database", skip(form, pool))]
async fn insert_subscriber(form: &FormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"insert into subscriptions(id, email, name, subscribed_at) values($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute the query : {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, db_pool_data),
    fields(
        subscriber_name = %form.name,
        subscriber_email = %form.email
        )
    )]
pub async fn subscribe(form: web::Form<FormData>, db_pool_data: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&form, db_pool_data.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
