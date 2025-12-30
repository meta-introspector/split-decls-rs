// Generated macro for impl_1377 (impl)
macro_rules! Depcrate_crypto_cipher_inboundimpl_1377 {
() => {
// Module: crate::crypto::cipher::inbound
// Provides: {"impl_1377"}
// Dependencies: {}
impl InboundPlainMessage < '_ > { # [doc = " Returns true if the payload is a CCS message."] # [doc = ""] # [doc = " We passthrough ChangeCipherSpec messages in the deframer without decrypting them."] # [doc = " Note: this is prior to the record layer, so is unencrypted. See"] # [doc = " third paragraph of section 5 in RFC8446."] pub (crate) fn is_valid_ccs (& self) -> bool { self . typ == ContentType :: ChangeCipherSpec && self . payload == [0x01] } }
};
}
