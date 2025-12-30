// Generated macro for impl_1431 (impl)
macro_rules! Depcrate_crypto_cipherimpl_1431 {
() => {
// Module: crate::crypto::cipher
// Provides: {"impl_1431"}
// Dependencies: {}
impl < 'a > Payload < 'a > { # [doc = " A reference to the payload's bytes"] pub fn bytes (& self) -> & [u8] { match self { Self :: Borrowed (bytes) => bytes , Self :: Owned (bytes) => bytes , } } pub (crate) fn into_owned (self) -> Payload < 'static > { Payload :: Owned (self . into_vec ()) } pub (crate) fn into_vec (self) -> Vec < u8 > { match self { Self :: Borrowed (bytes) => bytes . to_vec () , Self :: Owned (bytes) => bytes , } } pub (crate) fn read (r : & mut Reader < 'a >) -> Self { Self :: Borrowed (r . rest ()) } }
};
}
