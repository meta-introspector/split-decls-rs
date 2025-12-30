// Generated macro for server_ech_hrr_confirmation_secret (function)
macro_rules! Depcrate_tls13_key_scheduleserver_ech_hrr_confirmation_secret {
() => {
// Module: crate::tls13::key_schedule
// Provides: {"server_ech_hrr_confirmation_secret"}
// Dependencies: {}
pub (crate) fn server_ech_hrr_confirmation_secret (hkdf_provider : & 'static dyn Hkdf , client_hello_inner_random : & [u8] , hs_hash : hash :: Output ,) -> [u8 ; 8] { hkdf_expand_label (hkdf_provider . extract_from_secret (None , client_hello_inner_random) . as_ref () , SecretKind :: ServerEchHrrConfirmationSecret . to_bytes () , hs_hash . as_ref () ,) }
};
}
