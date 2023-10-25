use base64::Engine;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::{Error, Response};
use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use super::constants::{scope, auth_url};
use serde_urlencoded;
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: usize,
}

async fn auth() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("No client ID in ENV.");
    let redirect_uri = std::env::var("REDIRECT_URI").expect("No redirect URI in ENV.");
    let scope = scope();
    let auth_url = auth_url();

    let code_verifier = generate_code_verifier(128);
    let code_challenge = generate_code_challenge(&code_verifier);

    let auth_url = format!(
    "https://accounts.spotify.com/authorize?{}",
    serde_urlencoded::to_string([
        ("client_id", &client_id),
        ("response_type", &"code".to_string()),
        ("redirect_uri", &redirect_uri),
        ("code_challenge_method", &"S256".to_string()),
        ("code_challenge", &code_challenge),
        ("scope", &scope),
    ])?
);
    let authorization_code = todo!("Connect to frontend and acces url via web");
    let token_response = exchange_code(&authorization_code, &code_verifier).await?;

    let access_token = token_response.access_token.as_str().to_string();
    let refresh_token = token_response.refresh_token.as_str().to_string();
}

/// Generates and returns the authorization URL for Spotify's Web API.
///
/// # Returns
/// - `Ok(Arc<str>)` containing the authorization URL on success.
/// - `Err(Box<dyn std::error::Error>)` on failure.
///
/// # Panics
/// - If `CLIENT_ID` or `REDIRECT_URI` environment variables are not set.
async fn get_auth_url() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let client_id = std::env::var("CLIENT_ID").expect("No client ID in ENV.");
    let redirect_uri = std::env::var("REDIRECT_URI").expect("No redirect URI in ENV.");
    let code_challenge = generate_code_challenge(&generate_code_verifier(128));

    let auth_url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&code_challenge_method=S256&code_challenge={}",
        client_id,
        redirect_uri,
        code_challenge
);
    Ok(auth_url)
}

/// Exchanges the authorization code for access and refresh tokens.
///
/// This function sends a POST request to Spotify's token endpoint
/// with the necessary parameters to exchange the authorization code
/// for access and refresh tokens.
///
/// # Parameters
/// - `code`: The authorization code.
///
//// # Returns
/// - `Response` .
async fn exchange_code(code: &str, code_verifier: &str) -> Result<TokenResponse, Error> {
    let token_url = "https://accounts.spotify.com/api/token";
    let redirect_uri = std::env::var("REDIRECT_URI").expect("No redirect URI in ENV.");
    let params = [
        ("grant_type", "authorization code"),
        ("code", &code as &str),
        ("redirect_uri", &redirect_uri),
        ("code_verifier", &code_verifier as &str),
    ];

    let client = reqwest::Client::new();
    let res = client
        .post(token_url)
        .form(&params)
        .send()
        .await?
        .error_for_status()?.json::<TokenResponse>().await?;
    return Ok(res);
}

/// Generates a code challenge from a given code verifier.
///
/// # Parameters
/// - `input`: The code verifier.
///
/// # Returns
/// - An `Arc<str>` containing the code challenge.
fn generate_code_challenge(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(result)
}


/// Generates a code verifier of the specified length.
///
/// # Parameters
/// - `length`: The desired length of the code verifier.
///
/// # Returns
/// - An `Arc<str>` containing the code verifier.
fn generate_code_verifier(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime;

    #[test]
    fn test_generate_code_challenge() {
        let code_verifier = "some_code_verifier";
        let challenge = generate_code_challenge(code_verifier);
        assert!(!challenge.is_empty(), "Code challenge should not be empty");
    }

    #[test]
    fn test_generate_code_verifier() {
        let length = 128;
        let verifier = generate_code_verifier(length);
        assert_eq!(
            verifier.len(),
            length,
            "Code verifier length should match the specified length"
        );
    }

    #[test]
    fn test_get_auth_url() {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let result = get_auth_url().await;

            assert!(result.is_ok(), "Should return Ok");
            let url = result.unwrap();
            assert!(
                url.contains("https://accounts.spotify.com/authorize"),
                "URL should contain the authorization endpoint"
            );
        });
    }
}
