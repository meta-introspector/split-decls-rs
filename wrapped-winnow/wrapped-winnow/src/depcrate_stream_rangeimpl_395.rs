// Generated macro for impl_395 (impl)
macro_rules! Depcrate_stream_rangeimpl_395 {
() => {
// Module: crate::stream::range
// Provides: {"impl_395"}
// Dependencies: {}
impl From < core :: ops :: RangeToInclusive < usize > > for Range { # [inline (always)] fn from (range : core :: ops :: RangeToInclusive < usize >) -> Self { let start_inclusive = 0 ; let end_inclusive = Some (range . end) ; Self :: raw (start_inclusive , end_inclusive) } }
};
}
