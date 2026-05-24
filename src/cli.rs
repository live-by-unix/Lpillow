use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lpillow")]
#[command(version = "1.0.0")]
#[command(about = "LPillow 0% loss CAM")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { input, output } => {
            crate::compress::run(&input, &output)?;
        }
        Commands::Unpress { input, output } => {
            crate::unpress::run(&input, &output)?;
        }
    }

    Ok(())
}
