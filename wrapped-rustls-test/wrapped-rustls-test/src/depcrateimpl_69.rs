// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
impl MockClientVerifier { pub fn new (verified : fn () -> Result < PeerVerified , Error > , kt : KeyType , provider : & CryptoProvider ,) -> Self { Self { parent : webpki_client_verifier_builder (kt . client_root_store () , provider) . build () . unwrap () , verified , subjects : Arc :: from (kt . client_root_store () . subjects ()) , mandatory : true , offered_schemes : None , expect_raw_public_keys : false , raw_public_key_algorithms : Some (provider . signature_verification_algorithms) , } } }
};
}
