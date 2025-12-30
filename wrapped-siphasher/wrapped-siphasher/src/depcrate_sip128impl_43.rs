// Generated macro for impl_43 (impl)
macro_rules! Depcrate_sip128impl_43 {
() => {
// Module: crate::sip128
// Provides: {"impl_43"}
// Dependencies: {}
impl From < u128 > for Hash128 { fn from (v : u128) -> Self { Hash128 { h1 : v as u64 , h2 : (v >> 64) as u64 , } } }
};
}
