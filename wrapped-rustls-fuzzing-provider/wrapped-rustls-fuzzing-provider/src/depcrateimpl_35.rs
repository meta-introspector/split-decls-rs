// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl crypto :: hmac :: Hmac for Hmac { fn with_key (& self , _key : & [u8]) -> Box < dyn crypto :: hmac :: Key > { Box :: new (HmacKey) } fn hash_output_len (& self) -> usize { HASH_OUTPUT . len () } }
};
}
