use std::env;
use reqwest;

pub async fn is_hate_speech(text: String) -> bool {
    let url = env::var("HATE_SPEECH_URL").expect("Hate-speech api url was not set!");
    let client = reqwest::Client::new();
    let response = client.post(url)
        .body(text.clone())
        .send()
        .await
        .unwrap();

    if response.status() == reqwest::StatusCode::BAD_REQUEST {
        true
    }
    else {
        false
    }
}