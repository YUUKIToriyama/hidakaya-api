use actix_web::{middleware::Logger, App, HttpServer};

mod api;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(api::get_categories::get_categories)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
