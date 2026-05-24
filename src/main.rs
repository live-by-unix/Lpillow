mod audio;
mod compress;
mod fft;
mod ltf;
mod unpress;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lpillow")]
#[command(version = "1.0.0")]
#[command(about = "LPillow CAM Audio Transformer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        input: String,
        #[arg(short, long)]
        output: String,
    },
    Unpress {
        input: String,
        #[arg(short, long)]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { input, output } => {
            match compress::run(&input, &output) {
                Ok(_) => {
                    println!("compressed {} -> {}", input, output);
                }
                Err(e) => {
                    eprintln!("compress error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Unpress { input, output } => {
            match unpress::run(&input, &output) {
                Ok(_) => {
                    println!("unpressed {} -> {}", input, output);
                }
                Err(e) => {
                    eprintln!("unpress error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
