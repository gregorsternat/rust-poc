use clap::Args;
use alloy::{
    providers::{Provider, ProviderBuilder},
    primitives::Address,
    primitives::utils::format_ether,
};
use url::Url;

#[derive(Args)]
pub struct BalanceArgs {
    address: Address
}

pub async fn get_balance(args: BalanceArgs) -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/2BLjwpkbaoxWzIAlFlEm9oLHp2_9RUdk";
    let provider = ProviderBuilder::new().on_http(Url::parse(rpc_url)?);

    let wei_balance = provider.get_balance(args.address).await?;
    let eth_balance = format_ether(wei_balance);

    println!("balance of {} is {} eth", args.address, eth_balance);
    Ok(())
}