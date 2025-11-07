use derive_getters::Getters;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

use std::fmt::{Debug, Formatter};

#[derive(Getters)]
pub struct Wallet {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl Debug for Wallet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wallet")
            .field("secret_key", &self.secret_key.display_secret().to_string())
            .field("public_key", &self.public_key.to_string())
            .finish()
    }
}

pub fn generate_wallets(count: u8) -> Vec<Wallet> {
    let context = Secp256k1::new();
    let mut rng = rand::rng();

    (0..count)
        .map(|_| {
            let (secret_key, public_key) = context.generate_keypair(&mut rng);

            Wallet {
                secret_key,
                public_key,
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn display_wallets(wallets: &Vec<Wallet>) {
    wallets.into_iter().enumerate().for_each(|(index, wallet)| {
        println!("Wallet {}", index + 1);
        println!(
            "    Secret key: {}",
            wallet.secret_key.display_secret().to_string()
        );
        println!("    Public key: {}\n", wallet.public_key.to_string());
    });
}
