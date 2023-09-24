// The `prelude` module provides a convenient way to import a number
// of common dependencies at once. This can be useful if you are working
// with multiple parts of the library and want to avoid having
// to import each dependency individually.
use ethers::prelude::*;

// const RPC_URL: &str = "https://eth.llamarpc.com";
const RPC_URL: &str = "http://localhost:8545";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let block_number: U64 = provider.get_block_number().await?;
    println!("{block_number}");

    let price = provider.get_gas_price().await?;
    println!("price = {:#?}", price);

    let accounts = provider.get_accounts().await?;
    println!("accounts = {:#?}", accounts);

    // for account in accounts {
    //     let balance = provider.get_balance(account, None).await?;
    //
    //     println!("{}", format!("account = {account}, balance = {balance}"));
    // }


    let account_123 = accounts.get(1).unwrap();

    let balance = provider.get_balance(*account_123, None).await?;
    println!("{}", format!("account = {account_123}, balance = {balance}"));

    let result = provider.unlock_account(*account_123, "123".to_string(), None).await?;

    //0xea96591dd85741007f6fcd6822c451ca36d466c5 456
    println!("{}", result);

    let account_456 = accounts.get(2).unwrap();
    // let balance_456 = provider.get_balance(account_456, None).await?;

    // let tx = TransactionRequest::new().to(account_456).value(1000).from(account_123);
    let request = TransactionRequest::new();
    // provider.send_transaction()

    Ok(())
}
