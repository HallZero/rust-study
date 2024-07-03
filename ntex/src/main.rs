mod states;
mod routes;

use crate::states::app::AppState;
use ntex::web;


#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder{
    web::HttpResponse::Ok().body(req_body)
}

#[web::get("/test")]
async fn test(data: web::types::State<AppState>) -> impl web::Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}


// Main

#[ntex::main]
async fn main() -> std::io::Result<()>{
    web::HttpServer::new(|| {
        web::App::new()
            .state(AppState {
                app_name: String::from("Ntex")
            })
            .service(routes::greetings::hello)
            .service(echo)
            .service(test)   
            .service(
                web::scope("/extractors")
                    .service(routes::extractors::path_extractor)
                    .service(routes::extractors::query_extractor)
                    .service(routes::extractors::json_extractor)
            )
            .route("/hey", web::get().to(routes::greetings::manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}