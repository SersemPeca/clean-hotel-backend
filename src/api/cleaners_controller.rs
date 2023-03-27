use crate::db::util::DbPool;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::auth_utils::extraction::{TokenPayload, MaybeTokenPayload};
use crate::db::models::cleaner::NewCleaner;

#[post("/cleaners/take/{roomdId}")]
async fn take_room(
    req: HttpRequest,
    path_var: web::Path<i32>,
    MaybeTokenPayload(auth_payload): MaybeTokenPayload,
) -> impl Responder {
    use crate::db::crud::rooms::*;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let roomId: i32 = *path_var;

    let Some(room) = read_room_by_id(conns, roomId) else {
        return HttpResponse::InternalServerError().body("No such room exists");
    };

    let Some(TokenPayload { user_id, is_admin }) = auth_payload else {
        return HttpResponse::InternalServerError().body("No token provided");
    };

    match room.cleaner {
        None => {
            let successful = room.cleaner.is_none() && update_room_cleaner(conns, room.id, user_id).is_some();

            if successful {
                HttpResponse::Ok().body("Room taken successfully")
            } else {
                HttpResponse::InternalServerError().body("Could not take room")
            }
        },
        Some(_) => {
            HttpResponse::InternalServerError().body("Room already taken")
        },
    }
}


#[post("/cleaners/free/{roomdId}")]
async fn free_room(
    req: HttpRequest,
    path_var: web::Path<i32>,
    MaybeTokenPayload(auth_payload): MaybeTokenPayload,
) -> impl Responder {
    use crate::db::crud::rooms::*;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let roomId: i32 = *path_var;

    let Some(room) = read_room_by_id(conns, roomId) else {
        return HttpResponse::InternalServerError().body("No such room exists");
    };

    let Some(TokenPayload { user_id, is_admin }) = auth_payload else {
        return HttpResponse::InternalServerError().body("No token provided");
    };

    match room.cleaner {
        None => {
            HttpResponse::Ok().body("Room successfully freed")
        },
        Some(cleaner_id) => {
            if cleaner_id == user_id {
                let successful = free_room_by_id(conns, roomId).is_some();

                if successful {
                    HttpResponse::Ok().body("Successfully freed up room")
                } else {
                    HttpResponse::InternalServerError().body("Could not take room")
                }
            } else {
                //NOTE: This should NOT happen
                HttpResponse::InternalServerError().body("Cleaner isn't assigned to room")
            }
        },
    }
}

#[derive(Deserialize, Serialize)]
struct NewCleanerData {
    username: String,
    password: String,
    name: String,
}

//NOTE: For testing purposes only
#[post("cleaners/add")]
async fn add_cleaner(req: HttpRequest, body: web::Json<NewCleanerData>) -> impl Responder {
   use crate::api::util::{generate_salt, hash_password};
   use crate::db::crud::cleaners::create_cleaner;
   use crate::db::models::cleaner::NewCleaner;

   let Some(conns) = req.app_data::<DbPool>() else {
       return HttpResponse::InternalServerError().body("Could not connect to the DB");
   };

   let NewCleanerData { username, password, name } = &*body;

   let salt = generate_salt();

   let Ok(password_hash) = hash_password(password.clone(), salt) else {
       return HttpResponse::InternalServerError().body("Could not hash password");
   };

   let new_cleaner = NewCleaner {
       username,
       password: &password_hash,
       name,
   };

   match create_cleaner(conns, &new_cleaner) {
       Some(res) => {
           HttpResponse::Ok().json(res)
       }

       None => {
           HttpResponse::InternalServerError().body("Could not insert cleaner into database")
       }
   }


}
