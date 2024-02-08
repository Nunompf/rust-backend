use diesel::prelude::*;
use dotenvy::dotenv;
use r2d2::PooledConnection;
use std::env;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: r2d2::Pool<ConnectionManager<PgConnection>> = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
    };
}

pub fn establish_connection() {
    dotenv().ok();
    lazy_static::initialize(&POOL);

}

pub fn connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    POOL.get().unwrap()
}