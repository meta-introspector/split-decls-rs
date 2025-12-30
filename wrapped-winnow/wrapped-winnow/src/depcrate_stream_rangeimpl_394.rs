// Generated macro for impl_394 (impl)
macro_rules! Depcrate_stream_rangeimpl_394 {
() => {
// Module: crate::stream::range
// Provides: {"impl_394"}
// Dependencies: {}
impl From < core :: ops :: RangeInclusive < usize > > for Range { # [inline (always)] fn from (range : core :: ops :: RangeInclusive < usize >) -> Self { let start_inclusive = * range . start () ; let end_inclusive = Some (* range . end ()) ; Self :: raw (start_inclusive , end_inclusive) } }
};
}
