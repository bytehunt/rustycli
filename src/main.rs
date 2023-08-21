pub mod interface;
pub mod log;

#[tokio::main]
async fn main() {
    if let Err(err) = interface::run::run_rustycli().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
