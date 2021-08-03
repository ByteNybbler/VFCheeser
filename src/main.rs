use hyper::{Body, Client, Method, Request, Uri};
use hyper::body;
use hyper::client::HttpConnector;
use hyper::header::*;
use hyper::http::HeaderValue;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use url::form_urlencoded::byte_serialize;
use std::fs::File;
use std::io::prelude::*;

mod text_command;
use crate::text_command::*;

#[derive(Debug, Deserialize, Serialize)]
struct ResponseCreateAudio {
    mp3_loc: String,
    is_mobile: bool
}

struct AudioParameters {
    voice: String,
    voice_text: String,
    rate: i32,
    pitch: i32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut audio_parameters = AudioParameters {
        voice: query_string_encode("Dallas"),
        voice_text: query_string_encode("I'm liking video games."),
        rate: 170,
        pitch: 1
    };

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let stdin = std::io::stdin();

    let cookies = get_session_cookies(&client).await?;

    let mp3_bytes = get_mp3(&client, &audio_parameters, &cookies).await?;

    let mut file = File::create("test.mp3")?;
    file.write_all(&mp3_bytes)?;

    Ok(())
}

fn query_string_encode(query_parameter: &str) -> String {
    byte_serialize(query_parameter.as_bytes()).collect()
}

async fn get_mp3(client: &Client<HttpsConnector<HttpConnector>>, audio_parameters: &AudioParameters, cookies: &HeaderValue)
    -> Result<body::Bytes, Box<dyn std::error::Error + Send + Sync>> {
    let request = Request::builder()
        .method(Method::GET)
        .uri(get_audio_uri(&audio_parameters)?)
        .header(COOKIE, cookies)
        .body(Body::empty())?;
    
    let response_create_audio = client.request(request).await?;
    let response_create_audio_body_bytes = body::to_bytes(response_create_audio).await?;
    let response_create_audio_json: ResponseCreateAudio = serde_json::from_slice(&response_create_audio_body_bytes).unwrap();

    // TODO: Check for session expiration.

    let uri_mp3: Uri = format!(
        "https://www.cepstral.com/{}",
        response_create_audio_json.mp3_loc
    ).parse()?;
    let response_mp3 = client.get(uri_mp3).await?;
    let response_mp3_body_bytes = body::to_bytes(response_mp3).await?;

    // TODO: Determine if mp3 creation failed, perhaps via checking the response body for 404 keywords.

    let uri_delete_audio: Uri = "https://www.cepstral.com/demos/deleteAudio.php".parse()?;
    let request = Request::builder()
        .method(Method::POST)
        .uri(uri_delete_audio)
        .header(COOKIE, cookies)
        .body(Body::empty())?;

    client.request(request).await?;

    Ok(response_mp3_body_bytes)
}

async fn get_session_cookies(client: &Client<HttpsConnector<HttpConnector>>) -> Result<HeaderValue, Box<dyn std::error::Error + Send + Sync>> {
    let uri_cookies: Uri = "https://www.cepstral.com/en/demos".parse()?;
    let res = client.get(uri_cookies).await?;
    let cookies = &res.headers()[SET_COOKIE]; // Could panic if no Set-Cookie response header is present.
    Ok(cookies.clone())
}

fn get_audio_uri(audio_parameters: &AudioParameters) -> Result<Uri, hyper::http::uri::InvalidUri> {
    let uri = format!(
        "https://www.cepstral.com/demos/createAudio.php?voiceText={}&voice={}&createTime=1627763802118&rate={}&pitch={}&sfx=none",
        audio_parameters.voice_text,
        audio_parameters.voice,
        audio_parameters.rate,
        audio_parameters.pitch
    );
    uri.parse::<Uri>()
}