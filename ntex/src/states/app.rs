use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct AppState {
    pub app_name: String,
}