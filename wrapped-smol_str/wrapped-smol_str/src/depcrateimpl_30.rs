// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a > iter :: FromIterator < & 'a str > for SmolStr { fn from_iter < I : iter :: IntoIterator < Item = & 'a str > > (iter : I) -> SmolStr { build_from_str_iter (iter . into_iter ()) } }
};
}
