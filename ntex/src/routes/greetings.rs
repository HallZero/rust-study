use ntex::web;

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello World!")
}

pub async fn manual_hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hey There!")
}