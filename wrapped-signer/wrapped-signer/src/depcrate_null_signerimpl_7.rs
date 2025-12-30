// Generated macro for impl_7 (impl)
macro_rules! Depcrate_null_signerimpl_7 {
() => {
// Module: crate::null_signer
// Provides: {"impl_7"}
// Dependencies: {}
impl < T > PartialEq < T > for NullSigner where T : Signer , { fn eq (& self , other : & T) -> bool { self . pubkey == other . pubkey () } }
};
}
