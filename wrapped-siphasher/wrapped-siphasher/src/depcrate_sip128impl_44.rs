// Generated macro for impl_44 (impl)
macro_rules! Depcrate_sip128impl_44 {
() => {
// Module: crate::sip128
// Provides: {"impl_44"}
// Dependencies: {}
impl From < Hash128 > for u128 { fn from (h : Hash128) -> u128 { (h . h1 as u128) | ((h . h2 as u128) << 64) } }
};
}
