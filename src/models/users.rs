use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    //pub id: Uuid,
    pub email: String,
    pub name: String,
    pub user_id: String,
    pub subscribed_at: DateTime<Utc>,
}
