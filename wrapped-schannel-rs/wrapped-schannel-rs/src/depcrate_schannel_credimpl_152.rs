// Generated macro for impl_152 (impl)
macro_rules! Depcrate_schannel_credimpl_152 {
() => {
// Module: crate::schannel_cred
// Provides: {"impl_152"}
// Dependencies: {}
impl SchannelCred { # [doc = " Returns a builder."] pub fn builder () -> Builder { Builder :: new () } unsafe fn from_inner (inner : Credentials :: SecHandle) -> SchannelCred { SchannelCred (Arc :: new (RawCredHandle (inner))) } pub (crate) fn as_inner (& self) -> Credentials :: SecHandle { self . 0 . as_ref () . 0 } }
};
}
