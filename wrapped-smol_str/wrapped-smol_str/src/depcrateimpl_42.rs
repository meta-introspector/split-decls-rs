// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl From < SmolStr > for Arc < str > { # [inline (always)] fn from (text : SmolStr) -> Self { match text . 0 { Repr :: Heap (data) => data , _ => text . as_str () . into () , } } }
};
}
