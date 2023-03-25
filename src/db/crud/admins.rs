use crate::db::{
    models::admins::{Admin, NewAdmin},
    util::DbPool,
};
use diesel::prelude::*;

//TODO: Insert administrator profiles through different means
pub fn create_admin(conns: &DbPool, admin: &NewAdmin) -> Option<usize> {
    use crate::db::schema::admins;

        diesel::insert_into(admins::table)
        .values(admin)
        .returning(admins::id)
        .execute(&mut conns.get().unwrap())
        .ok()
}

//NOTE: For testing purposes
pub fn read_admins(conns: &DbPool) -> Option<Vec<Admin>> {
    use crate::db::schema::admins;

    admins::table
        .get_results::<Admin>(&mut conns.get().unwrap())
        .ok()
}

pub fn read_admin_by_username(conns: &DbPool, username: String) -> Option<Admin> {
    use crate::db::schema::admins;

    admins::table
        .filter(admins::username.eq(username))
        .get_result(&mut conns.get().unwrap())
        .ok()
}
