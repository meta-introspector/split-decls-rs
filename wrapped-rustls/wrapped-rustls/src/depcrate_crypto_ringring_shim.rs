// Generated macro for ring_shim (module)
macro_rules! Depcrate_crypto_ringring_shim {
() => {
// Module: crate::crypto::ring
// Provides: {"ring_shim"}
// Dependencies: {}
# [doc = " Compatibility shims between ring 0.16.x and 0.17.x API"] mod ring_shim { use ring :: agreement :: { self , EphemeralPrivateKey , UnparsedPublicKey } ; use crate :: crypto :: SharedSecret ; pub (super) fn agree_ephemeral (priv_key : EphemeralPrivateKey , peer_key : & UnparsedPublicKey < & [u8] > ,) -> Result < SharedSecret , () > { agreement :: agree_ephemeral (priv_key , peer_key , | secret | SharedSecret :: from (secret)) . map_err (| _ | ()) } }
};
}
