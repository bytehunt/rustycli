use crate::interface::args::Cli;
use clap::Parser;
use reqwest::{header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};

const PLAYGROUND_URL: &str = "https://play.rust-lang.org/execute";

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
#[serde(rename_all="camelCase")]
struct RequestPayload {
    channel: String,
    mode: String,
    edition: String,
    crateType: String, 
    tests: bool,
    backtrace: bool,
    code: String, 
}

#[derive(Debug, Deserialize)]
struct ResponsePayload {
    stdout: Option<String>,
    stderr: Option<String>,
}

impl RequestPayload {
    fn new(code: &str) -> RequestPayload {
        let cli = Cli::parse();
        RequestPayload {
            channel: cli.channel,
            mode: cli.mode,
            edition: cli.edition,
            crateType: "bin".to_string(),
            tests: false,
            backtrace: true,
            code: code.to_string(), 
        }
    }
}

pub async fn run_rustycli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let filename = &cli.run;

    let raw_code = std::fs::read_to_string(filename)?;
    let code = raw_code.trim();

    let request_payload = RequestPayload::new(code);

    let client = Client::new();
    let request_builder = client
        .post(PLAYGROUND_URL)
        .header(CONTENT_TYPE, "application/json")
        .json(&request_payload);

    // Get a string representation of the request and print it
    // let request_str = format!("{:#?}", request_builder);
    // println!("Request:\n{}", request_str);

    let resp_json = request_builder
        .send()
        .await?
        .json::<ResponsePayload>()
        .await?;

    if let Some(stdout) = resp_json.stdout {
        println!("{}", stdout);
    } else if let Some(stderr) = resp_json.stderr {
        println!("{}", stderr);
    } else {
        println!("Error: No stdout or stderr found in the response.");
    }

    Ok(()) // Return success
}
