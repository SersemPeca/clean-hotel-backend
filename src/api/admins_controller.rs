use crate::db::util::DbPool;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::auth_utils::extraction::{TokenPayload, MaybeTokenPayload};

#[derive(Deserialize)]
struct NewAdminData {
   username: String,
   password: String,
   name: String,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
struct Info {
    room_id: i32,
    cleaner_id: i32,
}

// http://localhost:8080/admins/assign/?rood_id=123,cleaner_id=123
#[post("/admins/assign/{info}")]
async fn admin_assign(
    req: HttpRequest,
    path_var: web::Path<Info>,
    TokenPayload { user_id, is_admin }: TokenPayload,
) -> impl Responder {
    use crate::db::crud::cleaners::read_cleaner_by_id;
    use crate::db::crud::rooms::{ read_room_by_id, update_room_cleaner };

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let path_info: Info = *path_var;

    let Some(room) = read_room_by_id(conns, path_info.room_id) else {
        return HttpResponse::InternalServerError().body("No such room exists");
    };

    let Some(cleaner) = read_cleaner_by_id(conns, path_info.cleaner_id) else {
        return HttpResponse::InternalServerError().body("No such cleaner exists");
    };

    if is_admin {
        let success = update_room_cleaner(conns, room.id, cleaner.id).is_some();

        if success {
            HttpResponse::Ok().body("Successfully updated room with new cleaner")
        } else {
            HttpResponse::InternalServerError().body("Unable to update room with new cleaner using admin endpoint")
        }
    } else {
        HttpResponse::Unauthorized().body("Only administrators can force-assign rooms to cleaners")
    }
}
