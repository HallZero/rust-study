use ntex::web;

struct AppState {
    app_name: String,
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
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}