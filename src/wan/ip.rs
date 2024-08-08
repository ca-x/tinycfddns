use reqwest::Error;
use anyhow::Result;

pub async fn get_public_ipv4() -> Result<String, Error> {
    let response = reqwest::Client::new()
        .get("https://api.ipify.org")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .await?
        .text()
        .await?;

    Ok(response.trim().to_string())
}


pub async fn get_public_ipv6() -> Result<String, Error> {
    let response = reqwest::Client::new()
        .get("https://api6.ipify.org")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .send()
        .await?
        .text()
        .await?;

    Ok(response.trim().to_string())
}
#[cfg(test)]
mod wan_test {
    use super::*;

    #[tokio::test]
    async fn test_get_public_ipv4() {
        let ipv4 = get_public_ipv4().await;
        println!("{:?}", ipv4);
    }


    #[tokio::test]
    async fn test_get_public_ipv6() {
        let ipv6 = get_public_ipv6().await;
        println!("{:?}", ipv6);
    }
}