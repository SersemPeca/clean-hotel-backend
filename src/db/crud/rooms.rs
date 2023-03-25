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

pub fn update_room_cleaner(conns: &DbPool, id: i32, cleaner_id: i32) -> Option<usize> {
    use crate::db::schema::rooms;

    diesel::update(rooms::table)
        .filter(rooms::id.eq(id))
        .set(rooms::cleaner.eq(cleaner_id))
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

pub fn read_rooms_by_cleaner_id(conns: &DbPool, cleaner_id: i32) -> Option<Vec<Room>> {
    use crate::db::crud::cleaners::read_cleaner_by_id;

    let cleaner = read_cleaner_by_id(conns, cleaner_id)?;

    Room::belonging_to(&cleaner)
        .get_results::<Room>(&mut conns.get().unwrap())
        .ok()
}

pub fn free_room_by_id(conns: &DbPool, id: i32) -> Option<usize> {
    use crate::db::schema::rooms;

    diesel::update(rooms::table)
        .filter(rooms::id.eq(id))
        .set(rooms::cleaner.eq::<Option<i32>>(None))
        .execute(&mut conns.get().unwrap())
        .ok()
}

// pub fn read_transactions_by_user_id(conns: &DbPool, user_id: i32) -> Option<Vec<Transaction>> {
//     use crate::db::schema::transactions;
// 
//     use crate::db::crud::users::read_user_by_id;
// 
//     let user = read_user_by_id(conns, user_id)?;
// 
//     UserTransaction::belonging_to(&user)
//         .inner_join(transactions::table)
//         .select(Transaction::as_select())
//         .get_results::<Transaction>(&mut conns.get().unwrap())
//         .ok()
// }

