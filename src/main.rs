pub mod interface;

fn main() {
    if let Err(err) = interface::run::run_rusty() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
