use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Run rust code using Rust Playground in your terminal."
)]
pub struct Cli {
    #[arg(short, long)]
    /// Speficy file to run code
    pub filename: String,

    #[arg(short, long, default_value = "stable")]
    /// Speficy which channel to run code on
    ///
    /// Available channels : stable, beta, nightly
    pub channel: String,

    #[arg(short, long, default_value = "debug")]
    /// Speficy mode to run code
    pub mode: String,

    #[arg(short, long, default_value = "2021")]
    /// Speficy rust edition
    ///
    /// Available editions: 2021, 2018, 2015
    pub edition: String,
}
