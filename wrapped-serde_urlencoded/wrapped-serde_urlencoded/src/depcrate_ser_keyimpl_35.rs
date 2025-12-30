// Generated macro for impl_35 (impl)
macro_rules! Depcrate_ser_keyimpl_35 {
() => {
// Module: crate::ser::key
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'key > Deref for Key < 'key > { type Target = str ; fn deref (& self) -> & str { match * self { Key :: Static (key) => key , Key :: Dynamic (ref key) => key , } } }
};
}
