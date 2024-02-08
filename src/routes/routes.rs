use crate::db::domain::person::*;
use crate::db::domain::wallet::*;
use crate::db::schema::wallet::{id, its};
use crate::error_handler::CustomError;
use crate::db::connection::{self, connection};

use actix_web::{ get, post,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
    web, App, Either, HttpRequest, HttpResponse, Result,
};

use diesel::dsl::today;
use diesel::expression::is_aggregate::No;
use diesel::result::Error;
use diesel::Connection;

type PgConnection = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>>;


#[get("")]
async fn get_persons() -> Result<HttpResponse, CustomError> {
    let persons = Person::get_all()?;
    Ok(HttpResponse::Ok().json(persons))
}

#[post("")]
async fn create_person() -> Result<HttpResponse, CustomError> {
    let mut conn = connection();

    let result = conn.transaction::<Person, CustomError, _>(|conn: &mut PgConnection | {
        let new_wallet = NewWallet{
            its: Some(0),
        };

        let wallet = Wallet::create_wallet_with_tx(new_wallet, conn)?;

        let new_person = NewPerson {
            wallet: wallet.id,
            created: Some(chrono::Utc::now().naive_utc())
        };

        let person = Person::create_person_with_tx(new_person, conn)?;

        return Ok(person);
    })?;

    Ok(HttpResponse::Created().json(result))
}


pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_persons);
    config.service(create_person);
}