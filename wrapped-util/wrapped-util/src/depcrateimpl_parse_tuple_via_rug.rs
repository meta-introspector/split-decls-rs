// Generated macro for impl_parse_tuple_via_rug (macro)
macro_rules! Depcrateimpl_parse_tuple_via_rug {
() => {
// Module: crate
// Provides: {"impl_parse_tuple_via_rug"}
// Dependencies: {}
# [allow (unused_macros)] # [cfg (not (feature = "build-mpfr"))] macro_rules ! impl_parse_tuple_via_rug { ($ ty : ty) => { impl ParseTuple for ($ ty ,) { fn parse (_input : & [& str]) -> Self { panic ! ("parsing this type requires the `build-mpfr` feature") } } impl ParseTuple for ($ ty , $ ty) { fn parse (_input : & [& str]) -> Self { panic ! ("parsing this type requires the `build-mpfr` feature") } } impl ParseTuple for ($ ty , i32) { fn parse (_input : & [& str]) -> Self { panic ! ("parsing this type requires the `build-mpfr` feature") } } impl ParseTuple for (i32 , $ ty) { fn parse (_input : & [& str]) -> Self { panic ! ("parsing this type requires the `build-mpfr` feature") } } impl ParseTuple for ($ ty , $ ty , $ ty) { fn parse (_input : & [& str]) -> Self { panic ! ("parsing this type requires the `build-mpfr` feature") } } } ; }
};
}
