use crate::structs::current_time;

pub struct Keypair {
    pub public_key: String,
    pub secret_key: String
}

pub fn generate_keypair(handle: &str) -> Keypair {
    let timestamp = current_time();

    KeyPair {
        public_key: format!("pub-{}-{}", handle, timestamp),
        secret_key: format!("sec-{}-{}", handle, timestamp),
    }
}

pub fn generate_did(public_key: &str) -> String {
    format!("did:key:{}", public_key)
}

pub fn sign_message(message: &str, secret_key: &str) -> String {
    format!(
        "sig-{}-{}", 
        message.chars().take(10).collect::<String>(), 
        secret_key.chars().take(8).collect::<String>()
    )
}

pub fn verify_signature(message: &str, signature: &str, _public_key: &str) -> bool {
    let expected_prefix = 
        format!("sig-{}", message.chars().take(10).collect::<String>());
        
    signature.starts_with(&expected_prefix)
} 