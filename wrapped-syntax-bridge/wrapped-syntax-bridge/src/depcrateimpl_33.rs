// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl < S : Copy , SM : SpanMapper < S > > SpanMapper < S > for & SM { fn span_for (& self , range : TextRange) -> S { SM :: span_for (self , range) } }
};
}
