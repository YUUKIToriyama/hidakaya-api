use actix_web::{middleware::Logger, App, HttpServer};

mod api;
mod model;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ロギングの設定
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Httpサーバーの立ち上げ
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
