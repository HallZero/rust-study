use ntex::web;

#[derive(Debug)]
struct MyError {
    name: &'static str,
}

#[web::get("/error")]
async fn error_test() -> Result<String, web::Error> {
    let result = Err(MyError {name: "test error"});

    result.map_err(| err | web::error::ErrorBadRequest(err.name).into())
}