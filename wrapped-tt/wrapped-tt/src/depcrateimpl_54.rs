// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl < Span : Copy > DelimSpan < Span > { pub fn from_single (sp : Span) -> Self { DelimSpan { open : sp , close : sp } } pub fn from_pair (open : Span , close : Span) -> Self { DelimSpan { open , close } } }
};
}
