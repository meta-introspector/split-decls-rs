// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a > iter :: FromIterator < & 'a String > for SmolStr { fn from_iter < I : iter :: IntoIterator < Item = & 'a String > > (iter : I) -> SmolStr { SmolStr :: from_iter (iter . into_iter () . map (| x | x . as_str ())) } }
};
}
