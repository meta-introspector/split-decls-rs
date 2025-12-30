// Generated macro for impl_49 (impl)
macro_rules! Depcrate_arbitraryimpl_49 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_49"}
// Dependencies: {}
impl WeightArg { fn is_zero (& self) -> bool { if let Expr :: Lit (lit) = & self . 0 { if let Lit :: Int (lit) = & lit . lit { if let Ok (value) = lit . base10_parse :: < u32 > () { return value == 0 ; } } } false } }
};
}
