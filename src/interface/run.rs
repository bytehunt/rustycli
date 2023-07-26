use crate::interface::args::Cli;
use clap::Parser;
use reqwest::{header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};

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
    // Constructor function to create a new RequestPayload instance
    // Takes a code string as input, and uses CLI arguments to populate other fields
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

    // Trim leading and trailing whitespaces from the code before executing it
    let code = raw_code.trim();
    let request_payload = RequestPayload::new(code);

    let client = Client::new();

    // Build the POST request with the playground URL, headers, and serialized JSON payload
    let request_builder = client
        .post(PLAYGROUND_URL)
        .header(CONTENT_TYPE, "application/json")
        .json(&request_payload);

    // Get a string representation of the request and print it
    // let request_str = format!("{:#?}", request_builder);
    // println!("{:?}", request_str);

    // Make the HTTP request to the playground and await the response
    let resp_json = request_builder
        .send()
        .await?
        .json::<ResponsePayload>()
        .await?;

    // println!("{:?}", resp_json);

    // Display the output of the executed code (if any) or show an error message
    match (resp_json.stdout, resp_json.stderr) {
        (Some(stdout), _) if !stdout.is_empty() => println!("{}", stdout),
        (_, Some(stderr)) => println!("{}", stderr),
        _ => println!("Error: No stdout or stderr found in the response."),
    }
    Ok(())
}
