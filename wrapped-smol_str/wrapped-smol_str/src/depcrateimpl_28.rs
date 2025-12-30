// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl iter :: FromIterator < String > for SmolStr { fn from_iter < I : iter :: IntoIterator < Item = String > > (iter : I) -> SmolStr { build_from_str_iter (iter . into_iter ()) } }
};
}
