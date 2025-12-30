// Generated macro for impl_54 (impl)
macro_rules! Depcrate_certificateimpl_54 {
() => {
// Module: crate::certificate
// Provides: {"impl_54"}
// Dependencies: {}
impl ValueOrd for Version { fn value_cmp (& self , other : & Self) -> der :: Result < Ordering > { (* self as u8) . value_cmp (& (* other as u8)) } }
};
}
