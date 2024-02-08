use chrono::NaiveDateTime;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use r2d2::PooledConnection;
use serde_derive::{Deserialize, Serialize};
use crate::db::schema::person;
use crate::db::connection as db;
use crate::error_handler::CustomError;

#[derive(Queryable, Selectable, QueryableByName)]
#[diesel(table_name = person)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub wallet: i32,
    pub created: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = person)]
pub struct NewPerson {
    pub wallet: i32,
    pub created: Option<NaiveDateTime>
}

impl Person {
    pub fn get_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection();
        let persons = diesel::sql_query("select * from person").get_results(&mut conn)?;
        Ok(persons)
    }

    pub fn create_person(new_person: NewPerson) -> Result<Self, CustomError> {
        let mut conn = db::connection();
        let person = diesel::insert_into(person::table)
            .values(new_person)
            .get_result(&mut conn)?;

        Ok(person)
    }

    pub fn create_person_with_tx(new_person: NewPerson, conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<Self, CustomError> {
        let person = diesel::insert_into(person::table)
            .values(new_person)
            .get_result(conn)?;

        Ok(person)
    }
}