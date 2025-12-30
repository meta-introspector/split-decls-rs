// Generated macro for impl_1120 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_pq_mlkemimpl_1120 {
() => {
// Module: crate::crypto::aws_lc_rs::pq::mlkem
// Provides: {"impl_1120"}
// Dependencies: {}
impl SupportedKxGroup for MlKem768 { fn start (& self) -> Result < StartedKeyExchange , Error > { let decaps_key = kem :: DecapsulationKey :: generate (& kem :: ML_KEM_768) . map_err (| _ | Error :: General ("key generation failed" . into ())) ? ; let pub_key_bytes = decaps_key . encapsulation_key () . and_then (| encaps_key | encaps_key . key_bytes ()) . map_err (| _ | Error :: General ("encaps failed" . into ())) ? ; Ok (StartedKeyExchange :: Single (Box :: new (Active { decaps_key : Box :: new (decaps_key) , encaps_key_bytes : Vec :: from (pub_key_bytes . as_ref ()) , }))) } fn start_and_complete (& self , client_share : & [u8]) -> Result < CompletedKeyExchange , Error > { let encaps_key = kem :: EncapsulationKey :: new (& kem :: ML_KEM_768 , client_share) . map_err (| _ | INVALID_KEY_SHARE) ? ; let (ciphertext , shared_secret) = encaps_key . encapsulate () . map_err (| _ | INVALID_KEY_SHARE) ? ; Ok (CompletedKeyExchange { group : self . name () , pub_key : Vec :: from (ciphertext . as_ref ()) , secret : SharedSecret :: from (shared_secret . as_ref ()) , }) } fn name (& self) -> NamedGroup { NamedGroup :: MLKEM768 } fn fips (& self) -> bool { super :: super :: fips () } }
};
}
