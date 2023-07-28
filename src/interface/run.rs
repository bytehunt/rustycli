use crate::interface::args::Cli;
use clap::Parser;
use reqwest::{header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};
use spinoff::{spinners, Color, Spinner};

const PLAYGROUND_URL: &str = "https://play.rust-lang.org/execute";

// This struct will be serialized to JSON before making the API call
#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
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

// Implementation of RequestPayload with a constructor function for easy creation
impl RequestPayload {
    fn new(code: &str) -> RequestPayload {
        let cli = Cli::parse();
        RequestPayload {
            channel: cli.channel,
            mode: cli.mode,
            edition: cli.edition,
            crateType: "bin".to_string(),
            tests: cli.tests,
            backtrace: cli.backtrace,
            code: code.to_string(),
        }
    }
}

pub async fn run_rustycli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let filename = &cli.run;
    let raw_code = tokio::fs::read_to_string(filename).await?;
    let spinner = Spinner::new(spinners::Dots, "Compiling... ", Color::Green);

    // Trim leading and trailing whitespaces from the code before executing it
    let code = raw_code.trim();
    let request_payload = RequestPayload::new(code);

    let client = Client::new();

    let build_request = client
        .post(PLAYGROUND_URL)
        .header(CONTENT_TYPE, "application/json")
        .json(&request_payload)
        .send()
        .await?
        .json::<ResponsePayload>()
        .await?;


    let match_result = match (build_request.stdout, build_request.stderr) {
        (Some(stdout), _) if !stdout.is_empty() => stdout,
        (_, Some(stderr)) => stderr,
        _ => String::from("Error: No stdout or stderr found in the response."),
    };

    spinner.success(&match_result);
    Ok(())
}
