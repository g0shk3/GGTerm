use ring::aead;
use ring::rand::{SecureRandom, SystemRandom};
use base64::{Engine as _, engine::general_purpose};

const KEY_LEN: usize = 32; // 256 bits for AES-256

#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Invalid data format")]
    InvalidFormat,
}

/// Get or create encryption key from environment or generate new one
/// In production, this should be stored securely (e.g., OS keychain)
fn get_encryption_key() -> [u8; KEY_LEN] {
    // TODO: In production, store this in OS keychain or secure storage
    // For now, using a static key (NOT SECURE FOR PRODUCTION)
    // This is just a placeholder - implement proper key management
    let mut key = [0u8; KEY_LEN];

    // Try to get from environment, otherwise use a default (INSECURE)
    if let Ok(key_str) = std::env::var("GGTERM_ENCRYPTION_KEY") {
        if let Ok(decoded) = general_purpose::STANDARD.decode(key_str) {
            if decoded.len() == KEY_LEN {
                key.copy_from_slice(&decoded);
                return key;
            }
        }
    }

    // Fallback: Use a derived key (STILL NOT SECURE FOR PRODUCTION)
    // In production, generate a random key on first run and store it securely
    let seed = b"ggterm_default_key_change_me!!"; // 32 bytes
    key.copy_from_slice(&seed[..]);
    key
}

/// Encrypt a password string
/// Returns base64-encoded "nonce:ciphertext" format
pub fn encrypt_password(password: &str) -> Result<String, EncryptionError> {
    if password.is_empty() {
        return Ok(String::new());
    }

    let key_bytes = get_encryption_key();
    let key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;
    let key = aead::LessSafeKey::new(key);

    // Generate random nonce
    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; 12]; // 96 bits for AES-GCM
    rng.fill(&mut nonce_bytes)
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

    // Encrypt
    let mut in_out = password.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

    // Combine nonce and ciphertext, then base64 encode
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&in_out);

    Ok(general_purpose::STANDARD.encode(&result))
}

/// Decrypt a password string
/// Expects base64-encoded "nonce:ciphertext" format
pub fn decrypt_password(encrypted: &str) -> Result<String, EncryptionError> {
    if encrypted.is_empty() {
        return Ok(String::new());
    }

    // Decode from base64
    let data = general_purpose::STANDARD
        .decode(encrypted)
        .map_err(|_| EncryptionError::InvalidFormat)?;

    if data.len() < 12 {
        return Err(EncryptionError::InvalidFormat);
    }

    // Split nonce and ciphertext
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let mut nonce_array = [0u8; 12];
    nonce_array.copy_from_slice(nonce_bytes);
    let nonce = aead::Nonce::assume_unique_for_key(nonce_array);

    let key_bytes = get_encryption_key();
    let key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;
    let key = aead::LessSafeKey::new(key);

    // Decrypt
    let mut in_out = ciphertext.to_vec();
    let plaintext = key
        .open_in_place(nonce, aead::Aad::empty(), &mut in_out)
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

    String::from_utf8(plaintext.to_vec())
        .map_err(|_| EncryptionError::DecryptionFailed("Invalid UTF-8".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let password = "my_secret_password_123";
        let encrypted = encrypt_password(password).unwrap();

        assert_ne!(encrypted, password);
        assert!(!encrypted.is_empty());

        let decrypted = decrypt_password(&encrypted).unwrap();
        assert_eq!(decrypted, password);
    }

    #[test]
    fn test_empty_password() {
        let encrypted = encrypt_password("").unwrap();
        assert_eq!(encrypted, "");

        let decrypted = decrypt_password("").unwrap();
        assert_eq!(decrypted, "");
    }

    #[test]
    fn test_different_encryptions() {
        let password = "test123";
        let enc1 = encrypt_password(password).unwrap();
        let enc2 = encrypt_password(password).unwrap();

        // Different nonces should produce different ciphertexts
        assert_ne!(enc1, enc2);

        // But both should decrypt to the same value
        assert_eq!(decrypt_password(&enc1).unwrap(), password);
        assert_eq!(decrypt_password(&enc2).unwrap(), password);
    }
}
