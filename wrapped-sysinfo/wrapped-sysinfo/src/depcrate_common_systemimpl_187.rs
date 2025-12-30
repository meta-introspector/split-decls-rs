// Generated macro for impl_187 (impl)
macro_rules! Depcrate_common_systemimpl_187 {
() => {
// Module: crate::common::system
// Provides: {"impl_187"}
// Dependencies: {}
impl UpdateKind { # [doc = " If `self` is `OnlyIfNotSet`, `f` is called and its returned value is returned."] # [allow (dead_code)] pub (crate) fn needs_update (self , f : impl Fn () -> bool) -> bool { match self { Self :: Never => false , Self :: Always => true , Self :: OnlyIfNotSet => f () , } } }
};
}
