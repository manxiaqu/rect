use futures;
use web3::{signing, types::TransactionParameters, Transport, Web3};

/// Common api for all transactions related
#[derive(Debug, Clone)]
pub struct Tx<T: Transport>(Web3<T>);

impl<T: Transport> Tx<T> {
    pub fn new(w3: Web3<T>) -> Self {
        Tx(w3)
    }
    /// sign and send raw transaction
    pub fn sign_and_send_raw_transaction<K: signing::Key>(
        &self,
        tx: TransactionParameters,
        k: K,
    ) -> web3::Result<()> {
        let signed_tx = futures::executor::block_on(self.0.accounts().sign_transaction(tx, k))?;
        // send raw transaction
        let result = self.0.eth().send_raw_transaction(signed_tx.raw_transaction);
        let result = futures::executor::block_on(result)?;
        println!("Transaction send success, tx: {:?}", result);

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {}
