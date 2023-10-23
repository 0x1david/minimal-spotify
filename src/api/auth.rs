use std::sync::Arc;

use base64::Engine;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::{Error, Response};
use sha2::{Digest, Sha256};


/// Generates and returns the authorization URL for Spotify's Web API.
///
/// # Returns
/// - `Ok(Arc<str>)` containing the authorization URL on success.
/// - `Err(Box<dyn std::error::Error>)` on failure.
///
/// # Panics
/// - If `CLIENT_ID` or `REDIRECT_URI` environment variables are not set.
async fn get_auth_url() -> Result<Arc<str>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // TODO: MOVE TO MAIN
    let client_id = std::env::var("CLIENT_ID").expect("No client ID in ENV.");
    let redirect_uri = std::env::var("REDIRECT_URI").expect("No redirect URI in ENV.");
    let code_challenge = generate_code_challenge(&generate_code_verifier(128));

    let auth_url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&code_challenge_method=S256&code_challenge={}",
        client_id,
        redirect_uri,
        code_challenge
);
    Ok(Arc::from(auth_url))
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
async fn exchange_code(code: Arc<str>, code_verifier: Arc<str>) -> Result<Response, Error> {
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
        .await
        .unwrap()
        .error_for_status()?;
    return Ok(res);

}


/// Generates a code challenge from a given code verifier.
///
/// # Parameters
/// - `input`: The code verifier.
///
/// # Returns
/// - An `Arc<str>` containing the code challenge.
fn generate_code_challenge(input: &str) -> Arc<str>{
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(result).into()
}

/// Generates a code verifier of the specified length.
///
/// # Parameters
/// - `length`: The desired length of the code verifier.
///
/// # Returns
/// - An `Arc<str>` containing the code verifier.
fn generate_code_verifier(length: usize) -> Arc<str>{
    let code_verifier: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    Arc::from(code_verifier)

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
        assert_eq!(verifier.len(), length, "Code verifier length should match the specified length");
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
            assert!(url.contains("https://accounts.spotify.com/authorize"), "URL should contain the authorization endpoint");
        });
    }
}
