use crate::db::util::DbPool;

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use crate::auth_utils::extraction::{TokenPayload};
use serde::{Deserialize, Serialize};

#[get("/rooms/all")]
async fn get_all_rooms(
    req: HttpRequest,
    TokenPayload { user_id: _, is_admin }: TokenPayload,
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

#[derive(Deserialize)]
struct NewRoomData {
    description: String,
}

// Actix will automatically require the presence of a token so there is no need for MaybeTokenPayload
#[post("/rooms/add")]
async fn add_room(
    req: HttpRequest,
    TokenPayload { user_id, is_admin }: TokenPayload,
    body: web::Json<NewRoomData>,
) -> impl Responder {
    use crate::db::crud::rooms::*;
    use crate::db::models::room::NewRoom;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let NewRoomData { description } = &*body;

    let new_room = NewRoom {
       cleaner: None, 
       description: Some(&description),
    };

    if is_admin {
        let res = create_room(conns, &new_room);

        match res {
            Some(rez) => {
                HttpResponse::Ok().json(rez)
            }
            _ => {
                HttpResponse::InternalServerError().body("Could not create new room!")
            }
        }
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

#[get("/rooms/free")]
async fn get_free_rooms(
    req: HttpRequest,
    //We only require the tokens presence
    _token: TokenPayload,
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

// Get rooms that still need cleaning
#[get("/rooms/dirty")]
async fn get_dirty_rooms(
    req: HttpRequest,
    _token: TokenPayload,
    ) -> impl Responder {
    use crate::db::crud::rooms::read_dirty_rooms;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let Some(rooms) = read_dirty_rooms(conns) else {
        return HttpResponse::InternalServerError().body("No dirty rooms");
    };

    HttpResponse::Ok().json(rooms)
}

#[get("/rooms/display")]
async fn get_rooms_to_display(
    req: HttpRequest,
    _token: TokenPayload,
    ) -> impl Responder {
    use crate::db::crud::rooms::read_rooms_to_display;

    let Some(conns) = req.app_data::<DbPool>() else {
        return HttpResponse::InternalServerError().body("Could not connect to database");
    };

    let Some(rooms) = read_rooms_to_display(conns) else {
        return HttpResponse::InternalServerError().body("No rooms to display");
    };

    HttpResponse::Ok().json(rooms)
}
