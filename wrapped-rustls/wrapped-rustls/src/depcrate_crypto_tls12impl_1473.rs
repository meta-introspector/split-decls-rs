// Generated macro for impl_1473 (impl)
macro_rules! Depcrate_crypto_tls12impl_1473 {
() => {
// Module: crate::crypto::tls12
// Provides: {"impl_1473"}
// Dependencies: {}
impl Prf for PrfUsingHmac < '_ > { fn for_key_exchange (& self , output : & mut [u8 ; 48] , kx : Box < dyn ActiveKeyExchange > , peer_pub_key : & [u8] , label : & [u8] , seed : & [u8] ,) -> Result < () , Error > { prf (output , self . 0 . with_key (kx . complete_for_tls_version (peer_pub_key , ProtocolVersion :: TLSv1_2) ? . secret_bytes () ,) . as_ref () , label , seed ,) ; Ok (()) } fn new_secret (& self , secret : & [u8 ; 48]) -> Box < dyn PrfSecret > { Box :: new (PrfSecretUsingHmac (self . 0 . with_key (secret))) } }
};
}
