// Generated macro for impl_392 (impl)
macro_rules! Depcrate_stream_rangeimpl_392 {
() => {
// Module: crate::stream::range
// Provides: {"impl_392"}
// Dependencies: {}
impl From < core :: ops :: RangeFrom < usize > > for Range { # [inline (always)] fn from (range : core :: ops :: RangeFrom < usize >) -> Self { let start_inclusive = range . start ; let end_inclusive = None ; Self :: raw (start_inclusive , end_inclusive) } }
};
}
