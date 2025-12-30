// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl crypto :: hmac :: Key for HmacKey { fn sign_concat (& self , _first : & [u8] , _middle : & [& [u8]] , _last : & [u8]) -> crypto :: hmac :: Tag { crypto :: hmac :: Tag :: new (HMAC_OUTPUT) } fn tag_len (& self) -> usize { HMAC_OUTPUT . len () } }
};
}
