//! Common types and cryptographic utilities for Pubky MVP
//!
//! This module provides:
//! - Keypair generation and management
//! - Public key serialization
//! - Signature creation and verification

use ed25519_dalek::{SigningKey, VerifyingKey, Signer as _, Verifier as _};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt;

pub use ed25519_dalek::Signature;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid public key format")]
    InvalidPublicKey,
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Base32 decode error: {0}")]
    Base32Error(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Ed25519 keypair for signing and verification
#[derive(Clone)]
pub struct Keypair {
    signing_key: SigningKey,
}

impl Keypair {
    /// Generate a new random keypair
    pub fn random() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        Self { signing_key }
    }
    
    /// Get the public key
    pub fn public_key(&self) -> PublicKey {
        PublicKey {
            verifying_key: self.signing_key.verifying_key(),
        }
    }
    
    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }
}

impl fmt::Debug for Keypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Keypair")
            .field("public_key", &self.public_key())
            .finish()
    }
}

/// Ed25519 public key
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicKey {
    verifying_key: VerifyingKey,
}

impl PublicKey {
    /// Create from bytes
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self> {
        let verifying_key = VerifyingKey::from_bytes(bytes)
            .map_err(|_| Error::InvalidPublicKey)?;
        Ok(Self { verifying_key })
    }
    
    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }
    
    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<()> {
        self.verifying_key
            .verify(message, signature)
            .map_err(|_| Error::InvalidSignature)
    }
    
    /// Encode as base32 string (z-base-32)
    pub fn to_z32(&self) -> String {
        base32::encode(base32::Alphabet::Z, &self.to_bytes())
    }
    
    /// Decode from base32 string
    pub fn from_z32(s: &str) -> Result<Self> {
        let bytes = base32::decode(base32::Alphabet::Z, s)
            .ok_or_else(|| Error::Base32Error("Invalid base32".to_string()))?;
        
        if bytes.len() != 32 {
            return Err(Error::Base32Error(format!(
                "Expected 32 bytes, got {}",
                bytes.len()
            )));
        }
        
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes);
        Self::from_bytes(&key_bytes)
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_z32())
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey({})", self.to_z32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keypair_generation() {
        let keypair = Keypair::random();
        let public_key = keypair.public_key();
        
        // Sign and verify
        let message = b"Hello, Pubky!";
        let signature = keypair.sign(message);
        assert!(public_key.verify(message, &signature).is_ok());
        
        // Wrong message should fail
        let wrong_message = b"Wrong message";
        assert!(public_key.verify(wrong_message, &signature).is_err());
    }
    
    #[test]
    fn test_public_key_encoding() {
        let keypair = Keypair::random();
        let public_key = keypair.public_key();
        
        // Encode and decode
        let encoded = public_key.to_z32();
        let decoded = PublicKey::from_z32(&encoded).unwrap();
        
        assert_eq!(public_key, decoded);
    }
    
    #[test]
    fn test_public_key_bytes() {
        let keypair = Keypair::random();
        let public_key = keypair.public_key();
        
        let bytes = public_key.to_bytes();
        let restored = PublicKey::from_bytes(&bytes).unwrap();
        
        assert_eq!(public_key, restored);
    }
}
