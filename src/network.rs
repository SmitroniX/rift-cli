use std::time::Duration;

use reqwest::blocking::Client;

pub fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("RIFT-CLI/0.1.0")
        .timeout(Duration::from_secs(15))
        .build()?;

    let response = client.get(url).send()?.error_for_status()?;

    Ok(response.text()?)
}
