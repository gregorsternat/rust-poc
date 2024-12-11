use clap::Args;
use alloy::{
    network::TransactionBuilder,
    primitives::{Address, keccak256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest
};
use url::Url;

#[derive(Args)]
pub struct CallArgs {
    to: Address,
    sig: String,
    args: Vec<String>
}

pub async fn call_contract(args: CallArgs) -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://eth-mainnet.g.alchemy.com/v2/2BLjwpkbaoxWzIAlFlEm9oLHp2_9RUdk";
    let provider = ProviderBuilder::new().on_http(Url::parse(rpc_url)?);

    let encoded_args = args.args.iter()
        .map(|arg| {
            let arg = arg.trim_start_matches("0x");
            hex::decode(arg).unwrap()
        })
        .flatten()
        .collect::<Vec<u8>>();

    let mut input_data = keccak256(args.sig.as_bytes()).to_vec();
    input_data.extend(encoded_args);

    let tx_request = TransactionRequest::default()
        .with_to(args.to)
        .with_input(input_data);
    let response = provider.call(&tx_request).await?;

    println!("{:?}", response);
    Ok(())
}
