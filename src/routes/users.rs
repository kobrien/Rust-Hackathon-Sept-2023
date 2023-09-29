use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use crate::db::{{ get_users as db_get_users, add_user as db_add_user }};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    user_id: String,
    email: String,
    name: String,
}

pub async fn add_user(db_pool: web::Data<PgPool>, item: web::Json<UserInput>) -> HttpResponse {
    //let user_input = item.into_inner();
    //let user_id = Uuid::new_v4(); // Generating a new UUID

    log::info!("adding user...\n{}", serde_json::to_string(&*item).unwrap_or_else(|e| format!("Error serializing item: {:?}", e)));

    match db_add_user(&db_pool, &item.user_id, &item.name, &item.email).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            log::error!("Failed to add user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}


pub async fn get_users(db_pool: web::Data<PgPool>) -> HttpResponse {
    //match crate::db::users::get_users(&db_pool).await {
    match db_get_users(&db_pool).await {
        Ok(users) => {
            let response = users.iter().map(|user| {
                // You can customize the format of the response as needed
                // For simplicity, we're just including user names here
                user.name.clone()
            }).collect::<Vec<String>>();
            HttpResponse::Ok().json(users)
        }
        Err(e) => {
            log::error!("Failed to retrieve users: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
