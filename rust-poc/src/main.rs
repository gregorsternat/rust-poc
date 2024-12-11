use clap::{Parser, Subcommand};
use commands::{
    balance::{get_balance, BalanceArgs},
    call::{call_contract, CallArgs}
};


mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand)]
enum Commands {
    Balance(BalanceArgs),
    Call(CallArgs)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.cmd {
        Commands::Balance(args) => get_balance(args).await,
        Commands::Call(args) => call_contract(args).await
    }
}