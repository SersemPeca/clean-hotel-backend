use crate::db::schema::rooms;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::db::models::cleaner::Cleaner;

#[derive(Associations, Identifiable, Queryable, Clone, Serialize, Deserialize, Debug)]
#[diesel(table_name = rooms)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Cleaner, foreign_key = cleaner))]
#[diesel(treat_none_as_null = true)]
pub struct Room {
    #[diesel(column_name = id)]
    pub id: i32,

    #[diesel(column_name = cleaner)]
    pub cleaner: Option<i32>,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Insertable)]
#[diesel(table_name = rooms)]
pub struct NewRoom {

    #[diesel(column_name = cleaner)]
    pub cleaner: Option<i32>,
}
