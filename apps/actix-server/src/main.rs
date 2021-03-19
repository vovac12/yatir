mod error;
mod folders;
mod prelude;
mod state;
mod users;
use crate::prelude::*;
use actix_web::{web, App, HttpServer};

fn init_logger() {
    dotenv::dotenv().ok();

    /*
    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "yatir=debug,actix_web=info");
    }
    */
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let app_data = web::Data::new(AppState::new());
    HttpServer::new(move || {
        App::new()
            .configure(error::configure_error_handlers)
            .service(web::scope("/folders").configure(folders::setup_service))
            .service(web::scope("/users").configure(users::setup_service))
            .app_data(app_data.clone())
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
