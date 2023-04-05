use crate::db::{
    models::room::{Room, NewRoom},
    util::DbPool,
};
use diesel::prelude::*;

pub fn create_room(conns: &DbPool, room: &NewRoom) -> Option<usize> {
    use crate::db::schema::rooms;

        diesel::insert_into(rooms::table)
        .values(room)
        .returning(rooms::id)
        .execute(&mut conns.get().unwrap())
        .ok()
}

pub fn read_rooms(conns: &DbPool) -> Option<Vec<Room>> {
    use crate::db::schema::rooms;

    rooms::table
        .get_results::<Room>(&mut conns.get().unwrap())
        .ok()
}

pub fn read_room_by_id(conns: &DbPool, id: i32) -> Option<Room> {
    use crate::db::schema::rooms;

    rooms::table
        .find(id)
        .get_result::<Room>(&mut conns.get().unwrap())
        .ok()
}

pub fn read_rooms_to_display(conns: &DbPool) -> Option<Vec<Room>> {
    use crate::db::schema::rooms;

    rooms::table
        .filter(rooms::cleaner.is_null())
        .filter(rooms::clean.eq(false))
        .get_results::<Room>(&mut conns.get().unwrap())
        .ok()
}

pub fn update_room_cleaner(conns: &DbPool, id: i32, cleaner_id: i32) -> Option<usize> {
    use crate::db::schema::rooms;

    diesel::update(rooms::table)
        .filter(rooms::id.eq(id))
        .set(
            (rooms::cleaner.eq(cleaner_id),
            rooms::clean.eq(false),
            ))
        .execute(&mut conns.get().unwrap())
        .ok()
}

pub fn update_room_to_be_cleaned_by_id(conns: &DbPool, id: i32) -> Option<usize> {
    use crate::db::schema::rooms;

    diesel::update(rooms::table)
        .filter(rooms::id.eq(id))
        .set(rooms::clean.eq(false))
        .execute(&mut conns.get().unwrap())
        .ok()
}

//NOTE: This is only used in the scheduler to update the rooms to be cleaned every 24 hours
pub fn update_all_rooms_to_be_cleaned(conns: &DbPool) -> Option<usize> {
    use crate::db::schema::rooms;

    diesel::update(rooms::table)
        .set(rooms::clean.eq(false))
        .execute(&mut conns.get().unwrap())
        .ok()
}

pub fn read_free_rooms(conns: &DbPool) -> Option<Vec<Room>> {
    use crate::db::schema::rooms;

    rooms::table
        .filter(rooms::cleaner.is_null())
        .get_results::<Room>(&mut conns.get().unwrap())
        .ok()
}

pub fn read_dirty_rooms(conns: &DbPool) -> Option<Vec<Room>> {
    use crate::db::schema::rooms;

    rooms::table
        .filter(rooms::clean.eq(false))
        .get_results::<Room>(&mut conns.get().unwrap())
        .ok()
}

pub fn read_rooms_by_cleaner_id(conns: &DbPool, cleaner_id: i32) -> Option<Vec<Room>> {
    use crate::db::crud::cleaners::read_cleaner_by_id;

    let cleaner = read_cleaner_by_id(conns, cleaner_id)?;

    Room::belonging_to(&cleaner)
        .get_results::<Room>(&mut conns.get().unwrap())
        .ok()
}

// This method frees the room AND sets it as "clean"
// NOTE: Single responsibility here is not followed, will fix in the future
pub fn free_room_by_id(conns: &DbPool, id: i32) -> Option<usize> {
    use crate::db::schema::rooms;

    diesel::update(rooms::table)
        .filter(rooms::id.eq(id))
        .set(
            (rooms::cleaner.eq::<Option<i32>>(None), 
            rooms::clean.eq::<bool>(true)),
              )
        .execute(&mut conns.get().unwrap())
        .ok()
}
