use crate::db::{
    models::cleaner::{Cleaner, NewCleaner},
    util::DbPool,
};
use diesel::prelude::*;

pub fn create_cleaner(conns: &DbPool, room: &NewCleaner) -> Option<usize> {
    use crate::db::schema::cleaners;

        diesel::insert_into(cleaners::table)
        .values(room)
        .returning(cleaners::id)
        .execute(&mut conns.get().unwrap())
        .ok()
}

pub fn read_cleaners(conns: &DbPool) -> Option<Vec<Cleaner>> {
    use crate::db::schema::cleaners;

    cleaners::table
        .get_results::<Cleaner>(&mut conns.get().unwrap())
        .ok()
}

pub fn read_cleaner_by_id(conns: &DbPool, id: i32) -> Option<Cleaner> {
    use crate::db::schema::cleaners;

    cleaners::table
        .find(id)
        .get_result::<Cleaner>(&mut conns.get().unwrap())
        .ok()
}

pub fn read_cleaner_by_name(conns: &DbPool, name: String) -> Option<Cleaner> {
    use crate::db::schema::cleaners;

    cleaners::table
        .filter(cleaners::name.eq(name))
        .get_result(&mut conns.get().unwrap())
        .ok()
}

pub fn read_cleaner_by_username(conns: &DbPool, username: String) -> Option<Cleaner> {
    use crate::db::schema::cleaners;

    cleaners::table
        .filter(cleaners::username.eq(username))
        .get_result(&mut conns.get().unwrap())
        .ok()
}
