mod server;
use server::run_server;

use tracing_subscriber;

use clap::{Parser, Subcommand};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 9023;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Serve {
        #[arg(long, default_value = DEFAULT_HOST)]
        host: String,

        #[arg(short, long, default_value_t = DEFAULT_PORT)]
        port: u16,
    },

    Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    match args.command {
        Commands::Serve { host, port } => {
            run_server(&host, port).await;
        }
        Commands::Client => {
            println!("🔧 Client mode not implemented yet");
        }
    }
}
