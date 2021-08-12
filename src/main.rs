use web3::types::{TransactionParameters, TransactionRequest, BlockId, BlockNumber};
use web3::ethabi::ethereum_types::{Address, U256};
use std::str::FromStr;
use web3::types::U64;
use secp256k1::SecretKey;
use std::io::Write;

fn main(){
    println!("Hello world!");
}


#[tokio::main]
//Returns the number of the most recent block and Get block data.
#[test]
async fn block_high() -> web3::Result<()>{
    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);

    let block_num = web3.eth().block_number().await?;
    println!("The number of the most recent block is : {:?}",block_num);
    let block_data = web3.eth().block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(block_num)))).await?;
    println!("The {:?} block data is： {:?}",block_num, block_data);

    //let mut file = std::fs::File::create("data.txt").expect("create failed");
    //file.write_all(block_data()).expect("write failed");
    //println!("data written to file" );
    Ok(())
}


#[tokio::main]
//Returns a list of addresses owned by client and the balance of the account of given address.
#[test]
async fn account_balance() -> web3::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("0x00Aa39d30F0D20FF03a22cCfc30B7EfbFca597C2".parse().unwrap());
    accounts.push("0x002e28950558fbede1a9675cb113f0bd20912019".parse().unwrap());
    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}


#[tokio::main]
#[test]
//从节点管理帐户发送事务
async fn send_transcation_test() -> web3::Result<()> {

    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    let balance_before = web3.eth().balance(accounts[1],None).await?;
    println!("Calling balance.");

    //通过TransacTIonRequest结构发送的事务的参数
    let tx_object = TransactionRequest {

        from: accounts[0],

        to: Some(accounts[1]),

        gas: None,

        gas_price: None,

        value: Some(U256::from(10000)),

        data: None,

        nonce: None,

        condition: None,

        transaction_type: None,
        access_list: None
    };

    let tx_hash = web3.eth().send_transaction(tx_object).await?;
    let balance_after = web3.eth().balance(accounts[1],None).await?;

    println!("tx_hash is:{}",tx_hash);
    println!("Balance before : {}",balance_before);
    println!("Balance after :{}",balance_after);

    Ok(())

}


#[tokio::main]
#[test]
async fn send_transcation_test2() -> web3::Result<()> {

    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("0x002e28950558fbede1a9675cb113f0bd20912019".parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }
        let from = Address::from_str("0x44408B7BcD29e32A4FB8538d49B51d0c25Fb3724").unwrap();



    // Insert the 20-byte "to" address in hex format
    let to = Address::from_str("0x00bd138abd70e2f00903268f3db08f2d25677c9e").unwrap();
    let private_key = SecretKey::from_str("f42675d15d32796a5d48b8358554701fc9e82a4599c5647d0f128a574eddb702").unwrap();
    let tx_object = TransactionParameters {
        nonce: Some(U256::from(0x13)),

        to: Option::from(to),
        value: U256::exp10(1),
        gas: U256::exp10(5),
        ..Default::default()
    };
    let signed = web3.accounts().sign_transaction(tx_object, &private_key).await?;
    let result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;
    println!("{}", result);
    Ok(())
}
