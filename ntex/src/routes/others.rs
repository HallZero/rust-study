use ntex::web;
use crate::states::app::AppState;

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder{
    web::HttpResponse::Ok().body(req_body)
}

#[web::get("/test")]
async fn test(data: web::types::State<AppState>) -> impl web::Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}