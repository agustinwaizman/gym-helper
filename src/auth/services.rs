use actix_web::{post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::MySqlPool;
use crate::auth::models::jwt_models::TokenType;
use crate::auth::models::requests::{RegisterRequest, LoginRequest};
use crate::auth::models::responses::{LoginResult, RefreshResult};
use crate::auth::models::users::UserRole;
use crate::auth::jwt::{generate_token, validate_token};
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
                    30,
                    user.id as usize,
                    format!("{:?}", user.role),
                    TokenType::Access,
                    data.jwt_secret.clone(),
                );
                let refresh_token = generate_token(
                    "Gym_Helper".to_string(),
                    user.username.clone(),
                    1440,
                    user.id as usize,
                    format!("{:?}", user.role),
                    TokenType::Refresh,
                    data.jwt_secret.clone(),
                );
                let response = LoginResult {
                    token: token,
                    refresh: refresh_token,
                };
                tracing::info!("User logged in: {}", req.username);
                HttpResponse::Ok().json(response)
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

#[post("/refresh_token")]
pub async fn refresh(
        refresh_jwt: Option<BearerAuth>,
        data: web::Data<crate::config::Config>) -> HttpResponse {
    let Some(refresh_jwt) = refresh_jwt else {
        return HttpResponse::Forbidden().body("Missing refresh token")
    };

    let claims = validate_token(refresh_jwt.token().to_string(), data.clone());

    match claims {
        Ok(claims) => match claims.token_type {
            TokenType::Access => {
                tracing::error!("Invalid token type: {:?}", claims.token_type);
                return HttpResponse::Unauthorized().body("Invalid token type")
            },
            TokenType::Refresh => {
                tracing::info!("Refresh token is valid");
                let new_token = generate_token(
                    claims.iss,
                    claims.sub.clone(),
                    30,
                    claims.user_id,
                    claims.role,
                    TokenType::Access,
                    data.jwt_secret.clone(),
                );
                tracing::info!("New token generated for user: {}", claims.sub);
                HttpResponse::Ok().json(RefreshResult{token: new_token})
            }
        },
        Err(err) => {
            tracing::error!("Error generating new token: {}", err);
            HttpResponse::Unauthorized().body("Error generating new token")
        }
    }
}
