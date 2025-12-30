// Generated macro for impl_1106 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_pq_hybridimpl_1106 {
() => {
// Module: crate::crypto::aws_lc_rs::pq::hybrid
// Provides: {"impl_1106"}
// Dependencies: {}
impl SupportedKxGroup for Hybrid { fn start (& self) -> Result < StartedKeyExchange , Error > { let classical = self . classical . start () ? . into_single () ; let post_quantum = self . post_quantum . start () ? . into_single () ; let combined_pub_key = self . layout . concat (post_quantum . pub_key () , classical . pub_key ()) ; Ok (StartedKeyExchange :: Hybrid (Box :: new (ActiveHybrid { classical , post_quantum , name : self . name , layout : self . layout , combined_pub_key , }))) } fn start_and_complete (& self , client_share : & [u8]) -> Result < CompletedKeyExchange , Error > { let (post_quantum_share , classical_share) = self . layout . split_received_client_share (client_share) . ok_or (INVALID_KEY_SHARE) ? ; let cl = self . classical . start_and_complete (classical_share) ? ; let pq = self . post_quantum . start_and_complete (post_quantum_share) ? ; let combined_pub_key = self . layout . concat (& pq . pub_key , & cl . pub_key) ; let secret = self . layout . concat (pq . secret . secret_bytes () , cl . secret . secret_bytes ()) ; Ok (CompletedKeyExchange { group : self . name , pub_key : combined_pub_key , secret : SharedSecret :: from (secret) , }) } fn name (& self) -> NamedGroup { self . name } fn fips (& self) -> bool { match self . layout . post_quantum_first { true => self . post_quantum . fips () , false => self . classical . fips () , } } }
};
}
