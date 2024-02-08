mod db;
mod routes;
mod error_handler;

use actix_web::{App, HttpServer, web};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    db::connection::establish_connection();

    HttpServer::new(|| App::new()
        .service(web::scope("/person")
            .configure(routes::init_routes)
        ),
    )
    
    .bind(("localhost", 3000))?
    .run()
    .await
}
