extern crate base64;
extern crate dotenv;
extern crate reqwest;

use base64::Engine;

async fn response<F>(get_body: F) -> Result<String, reqwest::Error>
where
    F: Fn() -> String,
{
    dotenv::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("No client ID in ENV.");
    let redirect_uri = std::env::var("REDIRECT_URI").expect("No redirect URI in ENV.");
    let client_secret = std::env::var("CLIENT_SECRET").expect("No client secret in ENV.");
    let auth = format!(
        "Basic {}",
        base64::engine::general_purpose::STANDARD
            .encode(format!("{}:{}", client_id, client_secret).as_bytes())
    );

    let client = reqwest::Client::new();
    client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", auth)
        .body("TODO")
        .send()
        .await?
        .text()
        .await //TODO: REPLACE UNWRAP
}

fn build_body() {}
