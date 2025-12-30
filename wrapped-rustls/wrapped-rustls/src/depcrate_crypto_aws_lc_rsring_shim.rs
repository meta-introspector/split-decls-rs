// Generated macro for ring_shim (module)
macro_rules! Depcrate_crypto_aws_lc_rsring_shim {
() => {
// Module: crate::crypto::aws_lc_rs
// Provides: {"ring_shim"}
// Dependencies: {}
# [doc = " Compatibility shims between ring 0.16.x and 0.17.x API"] mod ring_shim { use aws_lc_rs :: agreement :: { self , EphemeralPrivateKey , UnparsedPublicKey } ; use crate :: crypto :: SharedSecret ; pub (super) fn agree_ephemeral (priv_key : EphemeralPrivateKey , peer_key : & UnparsedPublicKey < & [u8] > ,) -> Result < SharedSecret , () > { agreement :: agree_ephemeral (priv_key , peer_key , () , | secret | { Ok (SharedSecret :: from (secret)) }) } }
};
}
