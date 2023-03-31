use actix_web::web;

use crate::api;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config
        // Cleaners Controller
        .service(api::cleaners_controller::take_room)
        .service(api::cleaners_controller::free_room)
        .service(api::cleaners_controller::add_cleaner)
        // Auth Controller
        .service(api::auth_controller::auth)
        // Rooms controller
        .service(api::rooms_controller::get_all_rooms)
        .service(api::rooms_controller::add_room)
        .service(api::rooms_controller::get_cleaner_rooms)
        .service(api::rooms_controller::get_free_rooms)
        .service(api::rooms_controller::get_room_by_id)
        // Admins Controller
        .service(api::admins_controller::admin_free_room)
        .service(api::admins_controller::admin_assign)
        ;
}

