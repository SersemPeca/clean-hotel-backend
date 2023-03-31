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

#[post("/admins/assign/roomid/{room_id}/cleanerid/{cleaner_id}")]
async fn admin_assign(
    req: HttpRequest,
    path: web::Path<Info>,
    TokenPayload { user_id , is_admin }: TokenPayload,
) -> impl Responder {
    use crate::db::crud::cleaners::read_cleaner_by_id;
    use crate::db::crud::rooms::{ read_room_by_id, update_room_cleaner };

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let path_info: Info = *path;

    let Some(room) = read_room_by_id(conns, path_info.room_id) else {
        return HttpResponse::InternalServerError().body("No such room exists");
    };

    let Some(cleaner) = read_cleaner_by_id(conns, path_info.cleaner_id) else {
        return HttpResponse::InternalServerError().body("No such cleaner exists");
    };

    if is_admin {
        let success = update_room_cleaner(conns, room.id, cleaner.id).is_some();

        if success {
            HttpResponse::Ok().json("Successfully updated room with new cleaner")
        } else {
            HttpResponse::InternalServerError().body("Unable to update room with new cleaner using admin endpoint")
        }
    } else {
        HttpResponse::Unauthorized().body("Only administrators can force-assign rooms to cleaners")
    }
}

#[post("/admins/free/room/{room_id}")]
async fn admin_free_room(
    req: HttpRequest,
    path: web::Path<i32>,
    TokenPayload { user_id , is_admin }: TokenPayload,
    ) -> impl Responder {

    use crate::db::crud::rooms::{ read_room_by_id, free_room_by_id };

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let room_id: i32 = *path;

    let Some(room) = read_room_by_id(conns, room_id) else {
        return HttpResponse::InternalServerError().body("No such room exists");
    };

    if is_admin {
        let success = free_room_by_id(conns, room.id).is_some();

        if success {
            HttpResponse::Ok().json("Successfully free up room")
        } else {
            HttpResponse::InternalServerError().body("Unable to free up room using admin endpoint")
        }
    } else {
        HttpResponse::Unauthorized().body("Only administrators can force-free rooms from cleaners!")
    }
}
