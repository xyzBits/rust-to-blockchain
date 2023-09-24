
#[tokio::main]
async fn main() -> web3::Result<()> {

    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);

    let mut accounts = web3.eth().accounts().await?;
    println!("accounts = {:#?}", accounts);

    Ok(())
}