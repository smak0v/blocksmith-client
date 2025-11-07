use secp256k1::{
    Message, Secp256k1, SecretKey,
    hashes::{Hash, sha256::Hash as Sha256Hash},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
    data: String,
    nonce: u64,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub transaction: Transaction,
    pub public_key: String,
}

impl Transaction {
    pub fn new(
        from: impl Into<String>,
        to: impl Into<String>,
        amount: u64,
        data: impl Into<String>,
        nonce: u64,
    ) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            amount,
            data: data.into(),
            nonce,
            signature: String::new(),
        }
    }

    pub fn sign(&mut self, secret_key: &SecretKey) {
        self.signature = Secp256k1::new()
            .sign_ecdsa(self.create_message(), secret_key)
            .to_string();
    }

    pub fn create_message(&self) -> Message {
        let data = json!({
            "from": self.from,
            "to": self.to,
            "amount": self.amount,
            "data": self.data,
            "nonce": self.nonce,
        });

        Message::from_digest(Sha256Hash::hash(data.to_string().as_bytes()).to_byte_array())
    }
}
