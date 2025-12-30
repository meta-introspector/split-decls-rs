// Generated macro for impl_26 (impl)
macro_rules! Depcrate_keypairimpl_26 {
() => {
// Module: crate::keypair
// Provides: {"impl_26"}
// Dependencies: {}
impl < K : KeypairRef > Keypair for K { type VerifyingKey = < Self as KeypairRef > :: VerifyingKey ; fn verifying_key (& self) -> Self :: VerifyingKey { self . as_ref () . clone () } }
};
}
