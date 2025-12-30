// Generated macro for impl_918 (impl)
macro_rules! Depcrate_util_readyimpl_918 {
() => {
// Module: crate::util::ready
// Provides: {"impl_918"}
// Dependencies: {}
impl < 'a , T , Request > Ready < 'a , T , Request > where T : Service < Request > , { # [allow (missing_docs)] pub fn new (service : & 'a mut T) -> Self { Self (ReadyOneshot :: new (service)) } }
};
}
