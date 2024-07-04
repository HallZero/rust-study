mod states;
mod routes;
mod error;

use crate::states::app::AppState;
use ntex::web;

async fn teste(req: web::HttpRequest) -> impl web::Responder {
    println!("{:?}", req);

    web::HttpResponse::Ok().body("Hello World!")
}

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

#[cfg(test)]
mod test {
    use super::*;
    use ntex::web::test;

    #[ntex::test]
    async fn test_index_get() {
        let app = test::init_service(
            web::App::new()
                .state(AppState { app_name: String::from("Ntex") })
                .route("/", web::get().to(teste)),
        )
        .await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        let body = test::read_body(resp).await;

        assert_eq!(body.as_ref(), b"Hello World!");
    }
}