mod states;
mod routes;
mod error;

use crate::states::app::AppState;
use ntex::web;

// Main

#[ntex::main]
async fn main() -> std::io::Result<()>{
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    web::HttpServer::new(|| {
        
        let logger = web::middleware::Logger::default();

        web::App::new()
            .wrap(logger)
            .state(AppState {
                app_name: String::from("Ntex")
            })
            .service(routes::greetings::hello)
            .service(routes::others::echo)
            .service(routes::others::test)   
            .service(
                web::scope("/extractors")
                    .service(routes::extractors::path_extractor)
                    .service(routes::extractors::query_extractor)
                    .service(routes::extractors::json_extractor)
            )
            .service(error::error_test)
            .route("/hey", web::get().to(routes::greetings::manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}