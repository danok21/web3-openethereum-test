use secp256k1::SecretKey;
use std::io::Write;
use std::str::FromStr;
use web3::ethabi::ethereum_types::U256;
use web3::types::{U64, RawTransaction};
use web3::types::{BlockId, BlockNumber, TransactionParameters, Address,TransactionRequest};
use web3::transports::Http;
use web3::api::Accounts;

/*
use web3：：futures：：Future;

use web3：：types：：Bytes;

use ethereum_tx_sign：：RawTransaction;

use ethereum_types：：{H160，H256，U256};
*/

fn main() {
    println!("Hello world!");
}


#[tokio::main]
//Returns the number of the most recent block and Get block data.
#[test]
async fn block_high() -> web3::Result<()>{
    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);

    let block_num = web3.eth().block_number().await?;
    println!("The number of the most recent block is : {:?}", block_num);
    let block_data = web3
        .eth()
        .block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(block_num))))
        .await?;
    println!("The {:?} block data is： {:?}", block_num, block_data);

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
    accounts.push(
        "0x00Aa39d30F0D20FF03a22cCfc30B7EfbFca597C2"
            .parse()
            .unwrap(),
    );
    accounts.push(
        "0x002e28950558fbede1a9675cb113f0bd20912019"
            .parse()
            .unwrap(),
    );
    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}


#[tokio::main]
#[test]
async fn send_tr() -> web3::Result {
    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);

    // Insert the 20-byte "from" address in hex format (prefix with 0x)
    let from = Address::from_str("0x004ec07d2329997267ec62b4166639513386f32e").unwrap();

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = Address::from_str("0x00bd138abd70e2f00903268f3db08f2d25677c9e").unwrap();

    // Build the tx object
    let tx_object = TransactionRequest {
        from,
        to: Some(to),
        value: Some(U256::exp10(17)), //0.1 eth
        ..Default::default()
    };

    // Send the tx to localhost
    let result = web3.eth().send_transaction(tx_object).await?;

    println!("Tx succeeded with hash: {}", result);

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
    let balance_before = web3.eth().balance(accounts[1], None).await?;
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
        access_list: None,
    };
    let tx_hash = web3.eth().send_transaction(tx_object).await?;
    let balance_after = web3.eth().balance(accounts[1], None).await?;

    println!("tx_hash is:{}", tx_hash);
    println!("Balance before : {}", balance_before);
    println!("Balance after :{}", balance_after);

    Ok(())
}


#[tokio::main]
#[test]
async fn send_transcation_test2() -> web3::Result<()> {

    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("0xecce1ce9ce3d41c32150ba94c16478c1894c6ecf".parse().unwrap());
    let acc =  accounts.clone();
    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }



    // Insert the 20-byte "to" address in hex format
    // let to = Address::from_str("0x00aa39d30f0d20ff03a22ccfc30b7efbfca597c2").unwrap();
    let mut i = 0;
    while i < acc.len()-1{
        let to = acc[i];
        let private_key = SecretKey::from_str("25d7d4e261abbca642d6f632f9ab78daf740ca63614badfcf809b3e0a8b034bd").unwrap();
        let tx_object = TransactionParameters {
            //nonce: Some(U256::from(0x15)),

            to: Option::from(to),
            value: U256::exp10(3),
            gas: U256::exp10(5),
            ..Default::default()
        };
        let signed = web3.accounts().sign_transaction(tx_object, &private_key).await?;
        let result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;
        println!("TX succeeded with hash:{}", result);
        i+=1;
    }
    Ok(())
}



#[tokio::main]
#[test]
async fn send_transaction_test3() -> web3::Result {
    // Sign up at infura > choose the desired network (eg Rinkeby) > copy the endpoint url into the below
    // If you need test ether use a faucet, eg https://faucet.rinkeby.io/
    let transport = web3::transports::Http::new("http://localhost:8540")?;
    let web3 = web3::Web3::new(transport);

    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("0xecce1ce9ce3d41c32150ba94c16478c1894c6ecf".parse().unwrap());

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = Address::from_str("0x00aa39d30f0d20ff03a22ccfc30b7efbfca597c2").unwrap();

    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let prvk = SecretKey::from_str("25d7d4e261abbca642d6f632f9ab78daf740ca63614badfcf809b3e0a8b034bd").unwrap();

    // Build the tx object
    let tx_object = TransactionParameters {
        to: Some(to),
        value: U256::exp10(1), //0.1 eth
        ..Default::default()
    };

    // Sign the tx (can be done offline)
    let signed = web3.accounts().sign_transaction(tx_object, &prvk).await?;

    // Send the tx to infura
    let result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;
    let balance_after = web3.eth().balance(accounts[accounts.len()-1], None).await?;

    println!("Tx succeeded with hash: {}", result);
    Ok(())
}


/*z
//签名发送交易
#[tokio::main]
#[test]
async fn send_transcation_test3() -> web3::Result<()> {

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

    let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().wait().unwrap();
    let balance_before = web3.eth().balance(accounts,None).wait().unwrap();
    let nonce = web3.eth().transaction_count(accounts[0], None).wait().unwrap();

    let tx = RawTransaction {
        raw: Default::default(),
        nonce: convert_u256(nonce),
        to: Some(convert_account(accounts[1]),
        value: U256::from(10000),
        gas_price: U256::from(1000000000),
        gas: U256::from(21000),
        data: Vec::new(),
        tx: Default::default()
    };

    let signed_tx = tx.sign(&get_private_key());
    let tx_hash = web3.eth().send_raw_transaction(Bytes::from(signed_tx).wait().unwrap();
    let balance_after = web3.eth().balance(accounts[1], None).wait().unwrap();
    println!("TX Hash: {:?}",tx_hash);
    println!("Balance before: {}", balance_before);
    println!("Balance after： {}", balance_after);



fn get_private_key() -> H256 {

        let private_key = hex::decode("4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d").unwrap();
        return H256(to_array(private_key.as_slice()))

    }

fn convert_u256（value: web3::types::U256)-> U256 {

        let web3::types::U256（ref arr) = value;
        let mut ret = [0; 4];
        ret［0］ = arr[0];
        ret[1] = arr[1];
        U256(ret)

    }

fn convert_account(value: web3::types::H160) -> H160 {

        let ret = H160::from(value.0);
        ret

    }

fn to_array = (bytes: &[u8] -> [u8; 32] {

        let mut array = [0;32];
        let bytes = &bytes[array.len()];
        array.copy_from_slice(bytes);
        array

    }
    Ok(())
}
*/









































//签名发送交易
#[tokio::main]
#[test]
async fn send_transcation_test3() -> web3::Result<()> {
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
    let to = Address::from_str("0x750F6288F094AC356474acBfDD556CE06E62b2e6").unwrap();
    let private_key = SecretKey::from_str("ac983270577bd276c2ff752765020196e93d6cf74d2e6504e7ec605c17be7e60").unwrap();
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
