use crate::db::util::DbPool;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::auth_utils::extraction::{TokenPayload, MaybeTokenPayload};

#[get("/rooms/all")]
async fn get_all_rooms(
    req: HttpRequest,
    TokenPayload { user_id, is_admin }: TokenPayload,
) -> impl Responder {
    use crate::db::crud::rooms::*;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let Some(rooms) = read_rooms(conns) else {
        return HttpResponse::InternalServerError().body("Could not get rooms");
    };

    HttpResponse::Ok().json(rooms)
}

#[get("/rooms/{id}")]
async fn get_room_by_id(
    req: HttpRequest,
    room_id: web::Path<i32>, 
    TokenPayload { user_id, is_admin }: TokenPayload,
) -> impl Responder {
    use crate::db::crud::rooms::*;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let Some(room) = read_room_by_id(conns, *room_id) else {
        return HttpResponse::InternalServerError().body("No such room exists");
    };

    HttpResponse::Ok().json(room)
}


// Actix will automatically require the presence of a token so there is no need for MaybeTokenPayload
#[post("/rooms/add")]
async fn add_room(
    req: HttpRequest,
    TokenPayload { user_id, is_admin }: TokenPayload,
) -> impl Responder {
    use crate::db::crud::rooms::*;
    use crate::db::models::room::NewRoom;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let new_room = NewRoom {
       cleaner: None, 
    };

    if is_admin {
        let res = create_room(conns, &new_room);

        HttpResponse::Ok().json(res)
    }
    else {
        HttpResponse::Unauthorized().body("You are not an administrator!")
    }
}

#[get("/rooms/my")]
async fn get_cleaner_rooms(
    req: HttpRequest,
    TokenPayload { user_id, ..}: TokenPayload,
) -> impl Responder {
    use crate::db::crud::rooms::*;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let Some(rooms) = read_rooms_by_cleaner_id(conns, user_id) else {
        return HttpResponse::InternalServerError().body("No such room exists");
    };

    HttpResponse::Ok().json(rooms)
}

#[get("rooms/free")]
async fn get_free_rooms(
    req: HttpRequest,
    //We only require the tokens presence
) -> impl Responder {
    use crate::db::crud::rooms::*;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let Some(rooms) = read_free_rooms(conns) else {
        return HttpResponse::InternalServerError().body("No such room exists");
    };

    HttpResponse::Ok().json(rooms)
}
