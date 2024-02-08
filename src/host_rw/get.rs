use reqwest;

#[allow(dead_code)]
pub async fn get(url: &str) -> Result<String, reqwest::Error> {
    println!("Downloading from {}", url);
    let body: String = reqwest::get(url).await?.text().await?;
    Ok(body)
}
