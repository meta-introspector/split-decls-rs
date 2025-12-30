// Generated macro for impl_521 (impl)
macro_rules! Depcrate_filters_pathimpl_521 {
() => {
// Module: crate::filters::path
// Provides: {"impl_521"}
// Dependencies: {}
impl Peek { # [doc = " Get the `&str` representation of the remaining path."] pub fn as_str (& self) -> & str { & self . path . path () [self . start_index ..] } # [doc = " Get an iterator over the segments of the peeked path."] pub fn segments (& self) -> impl Iterator < Item = & str > { self . as_str () . split ('/') . filter (| seg | ! seg . is_empty ()) } }
};
}
