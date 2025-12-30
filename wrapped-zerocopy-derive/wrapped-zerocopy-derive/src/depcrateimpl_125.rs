// Generated macro for impl_125 (impl)
macro_rules! Depcrateimpl_125 {
() => {
// Module: crate
// Provides: {"impl_125"}
// Dependencies: {}
impl Trait { fn crate_path (& self , zerocopy_crate : & Path) -> Path { match self { Self :: Sized => { parse_quote ! (# zerocopy_crate :: util :: macro_util :: core_reexport :: marker ::# self) } _ => parse_quote ! (# zerocopy_crate ::# self) , } } }
};
}
