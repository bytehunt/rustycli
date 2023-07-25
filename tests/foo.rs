//! ```cargo
//! [dependencies]
//! clap = { version = "4.2", features = ["derive"] }
//! ```

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
        #[arg(short, long, default_value = "PwnWriter")]
    pub name: String,

}

fn main() {
    let args = Args::parse();
    println!("{}", args.name);
}

