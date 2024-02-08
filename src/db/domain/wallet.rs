use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::PooledConnection;
use crate::{db::schema::wallet, error_handler::CustomError};
use serde_derive::{Deserialize, Serialize};
use crate::db::connection as db;

#[derive(Queryable, Selectable, QueryableByName)]
#[diesel(table_name = wallet)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub id: i32,
    pub its: Option<i32>
}
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = wallet)]
pub struct NewWallet {
    pub its: Option<i32>
}

impl Wallet {
    pub fn create_wallet(new_wallet: NewWallet) -> Result<Self, CustomError> {
        let mut conn = db::connection();
        let wallet = diesel::insert_into(wallet::table)
            .values(new_wallet)
            .get_result(&mut conn)?;

        Ok(wallet)
    }

    pub fn create_wallet_with_tx(new_wallet: NewWallet, conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<Self, CustomError> {
        let wallet = diesel::insert_into(wallet::table)
            .values(new_wallet)
            .get_result(conn)?;

        Ok(wallet)
    }
}