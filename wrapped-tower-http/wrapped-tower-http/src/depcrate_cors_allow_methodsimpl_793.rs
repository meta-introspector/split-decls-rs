// Generated macro for impl_793 (impl)
macro_rules! Depcrate_cors_allow_methodsimpl_793 {
() => {
// Module: crate::cors::allow_methods
// Provides: {"impl_793"}
// Dependencies: {}
impl < const N : usize > From < [Method ; N] > for AllowMethods { fn from (arr : [Method ; N]) -> Self { # [allow (deprecated)] Self :: list (array :: IntoIter :: new (arr)) } }
};
}
