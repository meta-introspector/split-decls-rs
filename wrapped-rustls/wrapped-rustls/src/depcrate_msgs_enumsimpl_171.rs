// Generated macro for impl_171 (impl)
macro_rules! Depcrate_msgs_enumsimpl_171 {
() => {
// Module: crate::msgs::enums
// Provides: {"impl_171"}
// Dependencies: {}
impl HpkeAead { # [doc = " Returns the length of the tag for the AEAD algorithm, or none if the AEAD is EXPORT_ONLY."] pub (crate) fn tag_len (& self) -> Option < usize > { match self { Self :: AES_128_GCM | Self :: AES_256_GCM | Self :: CHACHA20_POLY_1305 => Some (16) , _ => None , } } }
};
}
