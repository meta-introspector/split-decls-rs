// Generated macro for impl_38 (impl)
macro_rules! Depcrate_ser_keyimpl_38 {
() => {
// Module: crate::ser::key
// Provides: {"impl_38"}
// Dependencies: {}
impl < End , Ok > KeySink < End > where End : for < 'key > FnOnce (Key < 'key >) -> Result < Ok , Error > , { pub fn new (end : End) -> Self { KeySink { end } } }
};
}
