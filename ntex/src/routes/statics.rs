use ntex::web;
use ntex_files::NamedFile;

#[web::get("/{filename}*")]
async fn file_renderer(req: web::HttpRequest) -> Result<NamedFile, web::Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = NamedFile::open(path)?;
    Ok(file)
}


