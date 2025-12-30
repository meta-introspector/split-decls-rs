// Generated macro for impl_1375 (impl)
macro_rules! Depcrate_crypto_cipher_inboundimpl_1375 {
() => {
// Module: crate::crypto::cipher::inbound
// Provides: {"impl_1375"}
// Dependencies: {}
impl < 'a > BorrowedPayload < 'a > { # [doc = " Truncate the payload to `len` bytes."] pub fn truncate (& mut self , len : usize) { if len >= self . len () { return ; } self . 0 = core :: mem :: take (& mut self . 0) . split_at_mut (len) . 0 ; } pub (crate) fn into_inner (self) -> & 'a mut [u8] { self . 0 } pub (crate) fn pop (& mut self) -> Option < u8 > { if self . is_empty () { return None ; } let len = self . len () ; let last = self [len - 1] ; self . truncate (len - 1) ; Some (last) } }
};
}
