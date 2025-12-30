// Generated macro for impl_455 (impl)
macro_rules! Depcrate_msgs_persistimpl_455 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_455"}
// Dependencies: {}
impl Tls13ClientSessionValue { pub (crate) fn new (suite : & 'static Tls13CipherSuite , ticket : Arc < PayloadU16 > , secret : & [u8] , peer_identity : Identity < 'static > , server_cert_verifier : & Arc < dyn ServerVerifier > , client_creds : & Arc < dyn ClientCredentialResolver > , time_now : UnixTime , lifetime : Duration , age_add : u32 , max_early_data_size : u32 ,) -> Self { Self { suite , secret : Zeroizing :: new (PayloadU8 :: new (secret . to_vec ())) , age_add , max_early_data_size , common : ClientSessionCommon :: new (ticket , time_now , lifetime , peer_identity , server_cert_verifier , client_creds ,) , quic_params : PayloadU16 :: new (Vec :: new ()) , } } pub (crate) fn secret (& self) -> & [u8] { self . secret . 0 . as_ref () } pub fn max_early_data_size (& self) -> u32 { self . max_early_data_size } pub fn suite (& self) -> & 'static Tls13CipherSuite { self . suite } # [doc = " Test only: rewind epoch by `delta` seconds."] # [doc (hidden)] pub fn rewind_epoch (& mut self , delta : u32) { self . common . epoch -= delta as u64 ; } # [doc = " Test only: replace `max_early_data_size` with `new`"] # [doc (hidden)] pub fn _private_set_max_early_data_size (& mut self , new : u32) { self . max_early_data_size = new ; } pub fn set_quic_params (& mut self , quic_params : & [u8]) { self . quic_params = PayloadU16 :: new (quic_params . to_vec ()) ; } pub fn quic_params (& self) -> Vec < u8 > { self . quic_params . 0 . clone () } }
};
}
