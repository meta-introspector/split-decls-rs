// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl < S : Copy > Delimiter < S > { pub const fn invisible_spanned (span : S) -> Self { Delimiter { open : span , close : span , kind : DelimiterKind :: Invisible } } pub const fn invisible_delim_spanned (span : DelimSpan < S >) -> Self { Delimiter { open : span . open , close : span . close , kind : DelimiterKind :: Invisible } } pub fn delim_span (& self) -> DelimSpan < S > { DelimSpan { open : self . open , close : self . close } } }
};
}
