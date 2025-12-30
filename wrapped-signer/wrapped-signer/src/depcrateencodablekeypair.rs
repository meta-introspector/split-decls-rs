// Generated macro for EncodableKeypair (trait)
macro_rules! DepcrateEncodableKeypair {
() => {
// Module: crate
// Provides: {"EncodableKeypair"}
// Dependencies: {}
# [doc = " The `EncodableKeypair` trait extends `EncodableKey` for asymmetric keypairs, i.e. have"] # [doc = " associated public keys."] pub trait EncodableKeypair : EncodableKey { type Pubkey : ToString ; # [doc = " Returns an encodable representation of the associated public key."] fn encodable_pubkey (& self) -> Self :: Pubkey ; }
};
}
