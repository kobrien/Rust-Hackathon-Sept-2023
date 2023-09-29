use sqlx::PgPool;
use crate::models::User;
use uuid::Uuid;
use chrono::{Utc};

pub async fn get_users(db_pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db_pool)
        //.fetch_all(&**db_pool)
        .await
}

// pub async fn get_user_by_id(db_pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
//     sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = $1")
//     .bind(user_id)
//     .fetch_optional(db_pool)
//     .await
// }

pub async fn add_user(
    db_pool: &PgPool,
    user_id: &str,
    name: &str,
    email: &str,
) -> Result<(), sqlx::Error> {
    let uuid = Uuid::new_v4();
    let current_time = Utc::now(); // Get the current timestamp

    sqlx::query(
        "INSERT INTO users (id, user_id, name, email , subscribed_at) VALUES ($1, $2, $3, $4, $5)")
        //.bind(&Uuid::new_v4())
        .bind(uuid)
        .bind(user_id)
        .bind(name)
        .bind(email)
        .bind(current_time)
        .execute(db_pool)
        .await?;

    Ok(())
}
