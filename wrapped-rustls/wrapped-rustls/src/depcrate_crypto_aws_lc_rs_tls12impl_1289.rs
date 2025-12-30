// Generated macro for impl_1289 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_tls12impl_1289 {
() => {
// Module: crate::crypto::aws_lc_rs::tls12
// Provides: {"impl_1289"}
// Dependencies: {}
impl Prf for Tls12Prf { fn for_key_exchange (& self , output : & mut [u8 ; 48] , kx : Box < dyn ActiveKeyExchange > , peer_pub_key : & [u8] , label : & [u8] , seed : & [u8] ,) -> Result < () , Error > { Tls12PrfSecret { alg : self . 0 , secret : Secret :: KeyExchange (kx . complete_for_tls_version (peer_pub_key , ProtocolVersion :: TLSv1_2) ? ,) , } . prf (output , label , seed) ; Ok (()) } fn new_secret (& self , secret : & [u8 ; 48]) -> Box < dyn PrfSecret > { Box :: new (Tls12PrfSecret { alg : self . 0 , secret : Secret :: Master (Zeroizing :: new (* secret)) , }) } fn fips (& self) -> bool { super :: fips () } }
};
}
