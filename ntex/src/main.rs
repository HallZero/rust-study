use ntex::web;
use serde::Deserialize;

struct AppState {
    app_name: String,
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello World!")
}

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder{
    web::HttpResponse::Ok().body(req_body)
}

#[web::get("/test")]
async fn test(data: web::types::State<AppState>) -> impl web::Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

async fn manual_hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hey There!")
}


// Extractors


#[web::get("/users/{user_id}/{friend}")]
async fn path_extractor(
    path: web::types::Path<(String, String)>
) -> Result<String, web::Error> {
    let path = path.into_inner();
    Ok(format!("{} {}", path.0, path.1))
}

#[web::get("/query")]
async fn query_extractor(info: web::types::Query<Info>) -> String {
    format!("Welcome {}", info.username)
}

#[web::post("/json")]
async fn json_extractor(info: web::types::Json<Info>) -> Result<String, web::Error> {
    Ok(format!("Welcome {}!", info.username))
}


// Main


#[ntex::main]
async fn main() -> std::io::Result<()>{
    web::HttpServer::new(|| {
        web::App::new()
            .state(AppState {
                app_name: String::from("Ntex")
            })
            .service(
                web::scope("/testing")
                    .service(test)   
            )
            .service(hello)
            .service(echo)
            .service(path_extractor)
            .service(query_extractor)
            .service(json_extractor)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}