
use ethers::prelude::*;


const GETH_URL: &str = "http://localhost:8545";

#[tokio::main]
async fn main() -> eyre::Result<()> {

    let provider = Provider::<Http>::try_from(GETH_URL)?;

    let accounts  = provider.get_accounts().await?;
    println!("accounts = {:#?}", accounts);

    Ok(())
}