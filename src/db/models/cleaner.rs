use crate::db::schema::cleaners;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Clone, Serialize, Deserialize, Debug)]
#[diesel(table_name = cleaners)]
#[diesel(primary_key(id))]
pub struct Cleaner {
    // The ID of the cleaner
    #[diesel(column_name = id)]
    pub id: i32,

    // The username of the cleaner
    #[diesel(column_name = username)]
    pub username: String,

    // The password of the cleaner
    #[diesel(column_name = password)]
    pub password: String,

    // The name of the cleaner
    #[diesel(column_name = name)]
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = cleaners)]
pub struct NewCleaner<'a> {
    #[diesel(column_name = username)]
    pub username: &'a str,

    #[diesel(column_name = password)]
    pub password: &'a str,

    #[diesel(column_name = name)]
    pub name: &'a str,
}
