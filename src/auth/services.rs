use actix_web::{post, web, HttpResponse};
use sqlx::MySqlPool;
use crate::auth::models::{RegisterRequest, LoginRequest, UserRole};
use crate::auth::jwt::generate_token;
use crate::auth::handlers::{create_user_in_db, get_user_by_username, verify_password};


#[post("/register")]
pub async fn register(
    pool: web::Data<MySqlPool>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {

    let role = match req.role {
        UserRole::Admin => "Admin",
        UserRole::Trainer => "Trainer",
    };

    let result = create_user_in_db(&pool, &req.username, &req.password, role).await;

    match result {
        Ok(_) => {
            tracing::info!("User created successfully");
            HttpResponse::Created().body("User created successfully")
        },
        Err(e) => {
            tracing::error!("Error creating user: {}", e);
            HttpResponse::InternalServerError().body("Error creating user")
        }
    }
}

#[post("/login")]
async fn login(
    pool: web::Data<MySqlPool>,
    req: web::Json<LoginRequest>,
    data: web::Data<crate::config::Config>,
) -> HttpResponse {

    let user = get_user_by_username(&pool, &req.username).await;

    match user {
        Ok(Some(user)) => {
            if verify_password(&req.password, &user.hash) {
                let token = generate_token(
                    "Gym_Helper".to_string(),
                    user.username.clone(),
                    60,
                    user.id as usize,
                    format!("{:?}", user.role),
                    data.jwt_secret.clone(),
                );
                tracing::info!("User logged in: {}", req.username);
                HttpResponse::Ok().json(serde_json::json!({
                    "token": token,
                }))
            } else {
                tracing::info!("Invalid password for user: {}", req.username);
                HttpResponse::Unauthorized().body("Invalid password")
            }
        },
        Ok(None) => {
            tracing::error!("User not found: {}", req.username);
            HttpResponse::Unauthorized().body("Invalid username or password")
        },
        Err(err) => {
            tracing::error!("Error fetching user: {}", err);
            HttpResponse::InternalServerError().body("Error fetching user")
        }
    }
}