// The RUSTYCLI CLI

use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Pwnwriter < hey@Pwnwriter.xyz >",
    version,
    about,
    long_about = "🦊 Run rust code using Rust Playground in your terminal."
)]

pub struct Cli {
    #[arg(required = true, short, long)]
    /// Speficy file to run code
    pub run: String,

    #[arg(short, long, default_value = "stable")]
    /// Speficy which channel to run code on
    ///
    /// Available channels : stable, beta, nightly
    pub channel: String,

    #[arg(short, long, default_value = "debug")]
    /// Speficy the mode to run code
    pub mode: String,

    #[arg(short, long, default_value = "2021")]
    /// Speficy the rust edition
    ///
    /// Available editions: 2021, 2018, 2015
    pub edition: String,

    #[arg(short, long, default_value = "false")]
    /// Speficy backtrace t/f
    pub backtrace: bool,

    #[arg(short, long, default_value = "false")]
    /// Speficy tests t/f
    pub tests: String,
}
