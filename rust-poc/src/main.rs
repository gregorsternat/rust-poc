use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::Address;
use alloy::primitives::utils::format_ether;
use clap::Parser;
use url::Url;

#[derive(Parser)]
struct Cli {
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let rpc_url = "https://eth-mainnet.g.alchemy.com/<you key>";
    let provider = ProviderBuilder::new().on_http(Url::parse(rpc_url)?);

    let parsed_address = args.address.parse::<Address>()?;
    let wei_balance = provider.get_balance(parsed_address).await?;
    let eth_balance = format_ether(wei_balance);

    println!("balance of {} is {} eth", args.address, eth_balance);
    Ok(())
}