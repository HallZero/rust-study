use ntex::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

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