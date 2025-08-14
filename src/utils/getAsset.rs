#[cfg(feature = "web")]
use gloo_net::http::Request;
use dioxus::prelude::*;

#[cfg(feature = "web")]
pub async fn getAsset(filename: Asset) -> Result<String, String> {
    let response = Request::get(filename.to_string().as_str())
        .send()
        .await.unwrap();
    if response.ok() {
        return Ok(response.text().await.unwrap());
    } else {
        return Err(response.status_text());
    }
}

// #[cfg(feature = "desktop")]
// pub async fn getAsset(filename: String) -> Result<String, String> {
//     let bytes = std::fs::read(filename).unwrap();
// }