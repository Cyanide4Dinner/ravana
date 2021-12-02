pub mod oauth;

use reqwest::{ 
    Client,
    header::USER_AGENT,
};

#[tokio::main]
pub async fn call_api() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let res = client
        .get("https://www.reddit.com/r/rust/about.json")
        .header(USER_AGENT, "reddit-test-api (by u/OkAstronomer5277)")
        .send()
        .await?
        .text()
        .await?;
    println!("body = {:?}", res);
    Ok(())
}
