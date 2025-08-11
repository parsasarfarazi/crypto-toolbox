use crypto_bigint::{self, Random, U2048, rand_core::OsRng};
type Pint = U2048;

pub fn generate_keys() -> (PubKey, PrivKey) {
    todo!()
}

struct PubKey {
    public_key: Pint,
}

impl PubKey {
    pub fn encrypt(&self, plaintext: Pint) -> Pint {
        todo!()
    }
}

struct PrivKey {
    private_key: Pint,
}
impl PrivKey {
    pub fn decrypt(&self, ciphertext: Pint) -> Pint {
        todo!()
    }
}

mod test {
    use crate::rsa;
}
