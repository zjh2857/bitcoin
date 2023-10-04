use sha2::{Sha256, Digest};
pub fn string2hash(message: String) -> String {
    let mut hasher = Sha256::new();
    let bytes = message.as_bytes();
    hasher.update(bytes);
    let result = hasher.finalize();
    let vec = result.to_vec();
    let hash = hex::encode(vec);
    hash
}