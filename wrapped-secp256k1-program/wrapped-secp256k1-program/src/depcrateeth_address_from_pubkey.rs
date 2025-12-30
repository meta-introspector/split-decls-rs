// Generated macro for eth_address_from_pubkey (function)
macro_rules! Depcrateeth_address_from_pubkey {
() => {
// Module: crate
// Provides: {"eth_address_from_pubkey"}
// Dependencies: {}
# [doc = " Creates an Ethereum address from a secp256k1 public key."] pub fn eth_address_from_pubkey (pubkey : & [u8 ; SECP256K1_PUBKEY_SIZE] ,) -> [u8 ; HASHED_PUBKEY_SERIALIZED_SIZE] { let mut addr = [0u8 ; HASHED_PUBKEY_SERIALIZED_SIZE] ; addr . copy_from_slice (& sha3 :: Keccak256 :: digest (pubkey) [12 ..]) ; assert_eq ! (addr . len () , HASHED_PUBKEY_SERIALIZED_SIZE) ; addr }
};
}
