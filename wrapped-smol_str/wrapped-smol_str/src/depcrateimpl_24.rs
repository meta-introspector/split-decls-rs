// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl iter :: FromIterator < char > for SmolStr { fn from_iter < I : iter :: IntoIterator < Item = char > > (iter : I) -> SmolStr { from_char_iter (iter . into_iter ()) } }
};
}
