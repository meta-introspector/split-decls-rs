// Generated macro for impl_316 (impl)
macro_rules! Depcrate_typesimpl_316 {
() => {
// Module: crate::types
// Provides: {"impl_316"}
// Dependencies: {}
impl TestDesc { pub fn padded_name (& self , column_count : usize , align : NamePadding) -> String { let mut name = String :: from (self . name . as_slice ()) ; let fill = column_count . saturating_sub (name . len ()) ; let pad = " " . repeat (fill) ; match align { PadNone => name , PadOnRight => { name . push_str (& pad) ; name } } } # [doc = " Returns None for ignored test or tests that are just run, otherwise returns a description of the type of test."] # [doc = " Descriptions include \"should panic\", \"compile fail\" and \"compile\"."] pub fn test_mode (& self) -> Option < & 'static str > { if self . ignore { return None ; } match self . should_panic { options :: ShouldPanic :: Yes | options :: ShouldPanic :: YesWithMessage (_) => { return Some ("should panic") ; } options :: ShouldPanic :: No => { } } if self . compile_fail { return Some ("compile fail") ; } if self . no_run { return Some ("compile") ; } None } }
};
}
