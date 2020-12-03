use core::convert::From;
use core::str::FromStr;
use futures;
use secp256k1::SecretKey;
use std::time::Duration;
use web3::signing::SecretKeyRef;
use web3::types::{Address, TransactionParameters, U256};

// This is a example sending tx using web3.
#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);

    // key
    let secret_key =
        SecretKey::from_str("45dc3f2fce22a803e361d4f34257888d3d9a3e63056b43796cbdf8721e0c8e0d")
            .unwrap();
    let key = SecretKeyRef::from(&secret_key);
    let key_clone = key.clone();
    // sign transaction
    let mut tx_to_sign = TransactionParameters::default();
    tx_to_sign.to = Some(Address::from_str("fFbCB27d3A55698359cb7419275Be1877BDf918c").unwrap());
    tx_to_sign.value = U256::from(1000_000_000_000_000_000u64);

    let tx_to_sign_copy = tx_to_sign.clone();
    let signed_tx = web3.accounts().sign_transaction(tx_to_sign, key).await?;

    // send raw transaction
    let result = web3.eth().send_raw_transaction(signed_tx.raw_transaction);
    let result = futures::executor::block_on(result).unwrap();
    println!("Transaction send success, tx: {:?}", result);

    // send raw transaction with confirmation
    // nonce have changed
    let signed_tx = web3
        .accounts()
        .sign_transaction(tx_to_sign_copy, &key_clone)
        .await?;
    // only wait for 60s
    let receipt = web3
        .send_raw_transaction_with_confirmation(signed_tx.raw_transaction, Duration::new(60, 0), 0)
        .await?;

    println!(
        "Transaction {:?} execution result {:?}",
        receipt.transaction_hash, receipt.status
    );

    Ok(())
}
