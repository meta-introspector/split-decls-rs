// Generated macro for impl_28 (impl)
macro_rules! Depcrate_constfnimpl_28 {
() => {
// Module: crate::constfn
// Provides: {"impl_28"}
// Dependencies: {}
impl Qualifiers { fn from_ident (ident : & Ident) -> Self { match ident . to_string () . as_str () { "async" => Qualifiers :: Async , "unsafe" => Qualifiers :: Unsafe , "extern" => Qualifiers :: Extern , _ => Qualifiers :: None , } } }
};
}
