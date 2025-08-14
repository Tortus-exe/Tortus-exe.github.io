use gloo_net::http::Request;

#[cfg(feature = "web")]
pub async fn getAsset(filename: String) -> Result<String, String> {
    let response = Request::get(&filename)
        .send()
        .await.unwrap();
    if response.ok() {
        return Ok(response.text().await.unwrap());
    } else {
        return Err(response.status_text());
    }
}