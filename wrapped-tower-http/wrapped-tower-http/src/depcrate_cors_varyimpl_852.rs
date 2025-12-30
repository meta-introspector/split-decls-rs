// Generated macro for impl_852 (impl)
macro_rules! Depcrate_cors_varyimpl_852 {
() => {
// Module: crate::cors::vary
// Provides: {"impl_852"}
// Dependencies: {}
impl < const N : usize > From < [HeaderName ; N] > for Vary { fn from (arr : [HeaderName ; N]) -> Self { # [allow (deprecated)] Self :: list (array :: IntoIter :: new (arr)) } }
};
}
