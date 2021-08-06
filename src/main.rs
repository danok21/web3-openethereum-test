fn main(){
    high();
    println!("\n--------------------------------------------------------\n");
    account_balance();
    println!("Hello world!");
}


#[tokio::main]
//#[test] //show block height。
async fn high() -> web3::Result<()>{
    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);

    let block_num = web3.eth().block_number().await?;
    println!("block height : {:?}",block_num);

    Ok(())
}




#[tokio::main]
//#[test] //accounts and balance.
async fn account_balance() -> web3::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("0x00Aa39d30F0D20FF03a22cCfc30B7EfbFca597C2".parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}