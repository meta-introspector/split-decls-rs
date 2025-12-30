// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < S > Ident < S > { pub fn new (text : & str , span : S) -> Self { let (is_raw , text) = IdentIsRaw :: split_from_symbol (text) ; Ident { sym : Symbol :: intern (text) , span , is_raw } } }
};
}
