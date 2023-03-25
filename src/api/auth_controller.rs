use crate::db::util::DbPool;
use crate::auth_utils::extraction::Claims;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AuthInfo {
    username: String,
    password: String,
}

#[post("/auth/login")]
async fn auth(req: HttpRequest, body: web::Json<AuthInfo>) -> impl Responder {
    use crate::api::util::verify_password;
    use crate::db::crud::cleaners::read_cleaner_by_username;
    use crate::db::crud::admins::read_admin_by_username;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to the DB");
    };

    let AuthInfo { username, password } = &*body;

    struct Login { id: i32, password: String, is_admin: bool }

    let Some(user) = Option::or(
            read_admin_by_username(conns, username.to_string())
                .map(|admin| Login {
                    id: admin.id,
                    password: admin.password,
                    is_admin: true,
                }),
            read_cleaner_by_username(conns, username.to_string())
                .map(|cleaner| Login {
                    id: cleaner.id,
                    password: cleaner.password,
                    is_admin: false,
                })
    ) else {
        return HttpResponse::NotFound().body(format!("No such user {}", username));
    };

    match verify_password(password.to_string(), user.password) {
        Ok(_) => {
            let exp: usize = (Utc::now() + Duration::hours(1)).timestamp() as usize;
            let claims: Claims = Claims {
                exp,
                user_id: user.id,
                is_admin: user.is_admin
            };

            let secret: &str = &std::env::var("JWT_SECRET").expect("Missing ${JWT_SECRET}");

            let token: String = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )
            .unwrap();

            #[derive(Serialize)]
            struct EncodeResponse {
                message: String,
                token: String,
            }

            HttpResponse::Ok().json(EncodeResponse {
                message: format!(
                    "Successfully authenticated. Token will be valid until (epoch) {}",
                    exp
                ),
                token,
            })
        }
        Err(_) => HttpResponse::InternalServerError().body("Wrong username/password"),
    }
}
