// Generated macro for Decrypted (struct)
macro_rules! Depcrate_record_layerDecrypted {
() => {
// Module: crate::record_layer
// Provides: {"Decrypted"}
// Dependencies: {}
# [doc = " Result of decryption."] # [derive (Debug)] pub (crate) struct Decrypted < 'a > { # [doc = " Whether the peer appears to be getting close to encrypting too many messages with this key."] pub (crate) want_close_before_decrypt : bool , # [doc = " The decrypted message."] pub (crate) plaintext : InboundPlainMessage < 'a > , }
};
}
