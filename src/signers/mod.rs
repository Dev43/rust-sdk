use crate::{error::BundlrError, index::SignerMap};
use bytes::Bytes;

#[cfg(feature = "arweave")]
pub mod arweave;
#[cfg(feature = "cosmos")]
pub mod cosmos;
#[cfg(any(feature = "solana", feature = "algorand"))]
pub mod ed25519;
#[cfg(any(feature = "ethereum", feature = "erc20"))]
pub mod secp256k1;

pub trait ToPem {}

pub trait Signer {
    fn sign(&self, message: Bytes) -> Result<Bytes, BundlrError>;
    fn sig_type(&self) -> SignerMap;
    fn get_sig_length(&self) -> u16;
    fn get_pub_length(&self) -> u16;
    fn pub_key(&self) -> Bytes;
}
