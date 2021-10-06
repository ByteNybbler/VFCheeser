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

pub mod macro_assist;
pub mod parse;
pub mod text_command;
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

struct VoiceforgeCheeser {
    audio_parameters: AudioParameters,
    running: bool
}

impl VoiceforgeCheeser {
    pub fn new() -> VoiceforgeCheeser {
        VoiceforgeCheeser {
            audio_parameters: AudioParameters {
                voice: query_string_encode("Dallas"),
                voice_text: query_string_encode("I'm liking video games."),
                rate: 170,
                pitch: 1
            },
            running: true
        }
    }

    text_commands! {
        fn exit(&mut self) {
            self.running = !self.running;
        }

        #[alias(p)]
        fn pitch(&mut self, new_pitch: i32) {
            self.audio_parameters.pitch = new_pitch;
            notify_value_change("pitch", self.audio_parameters.pitch);
        }

        #[alias(r)]
        fn rate(&mut self, new_rate: i32) {
            self.audio_parameters.rate = new_rate;
            notify_value_change("rate", self.audio_parameters.rate);
        }

        #[alias(v)]
        fn voice(&mut self, new_voice: &str) {
            self.audio_parameters.voice = query_string_encode(new_voice);
            notify_value_change("voice", new_voice);
        }
    }
}

fn notify_value_change<T>(name: &str, value: T)
where
    T: std::fmt::Display
{
    println!("{} set to {}", name, value);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut cheeser = VoiceforgeCheeser::new();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let stdin = std::io::stdin();

    //let message = "You are the worst of the best.";
    //let message = "/rate 5";
    //let message = "/rate";
    let message = "/r 5";
    //let message = "/voice Alice";

    if let Some(command) = TextCommand::from_str(message, '/') {
        match cheeser.run_command(&command) {
            Ok(()) => (),
            Err(e) => {
                println!("{} error: {}", command.name(), e);
                if e.is_usage_error() {
                    VoiceforgeCheeser::help(Some(command.name()));
                }
            }
        }
    } else {
        cheeser.audio_parameters.voice_text = query_string_encode(&message);

        let cookies = get_session_cookies(&client).await?;

        println!("{}", cheeser.audio_parameters.pitch);

        let mp3_bytes = get_mp3(&client, &cheeser.audio_parameters, &cookies).await?;
    
        let mut file = File::create("test.mp3")?;
        file.write_all(&mp3_bytes)?; 
    }

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