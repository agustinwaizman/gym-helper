use actix_web::{post, web, HttpResponse};
use sqlx::MySqlPool;
use crate::auth::models::{RegisterRequest, LoginRequest, User, UserRole};
use crate::auth::jwt::generate_token;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, PasswordHash, PasswordVerifier, rand_core::OsRng};


fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash);
    match parsed_hash {
        Ok(ph) => Argon2::default()
            .verify_password(password.as_bytes(), &ph)
            .is_ok(),
        Err(_) => false,
    }
}

#[post("/register")]
pub async fn register(
    pool: web::Data<MySqlPool>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    let hashed = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Error hashing password")
    };

    let role = match req.role {
        UserRole::Admin => "Admin",
        UserRole::Trainer => "Trainer",
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, hash, role) VALUES (?, ?, ?)
        "#,
        req.username,
        hashed,
        role)
            .execute(pool.get_ref())
            .await;

    match result {
        Ok(_) => HttpResponse::Created()
            .body("User created"),
        Err(err) => {
            tracing::error!("Error creating user: {}", err);
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
    let user_result = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            hash,
            role as "role: UserRole",
            created_at,
            updated_at
        FROM users
        WHERE username = ?
        "#,
        req.username
    )
    .fetch_optional(pool.get_ref())
    .await;

    match user_result {
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
                HttpResponse::Ok().json(serde_json::json!({
                    "token": token,
                }))
            } else {
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
