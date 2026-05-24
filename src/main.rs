mod cli;
mod compress;
mod ltf;
mod unpress;
mod utils;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
