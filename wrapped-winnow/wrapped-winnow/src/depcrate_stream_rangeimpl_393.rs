// Generated macro for impl_393 (impl)
macro_rules! Depcrate_stream_rangeimpl_393 {
() => {
// Module: crate::stream::range
// Provides: {"impl_393"}
// Dependencies: {}
impl From < core :: ops :: RangeTo < usize > > for Range { # [inline (always)] fn from (range : core :: ops :: RangeTo < usize >) -> Self { let start_inclusive = 0 ; let end_inclusive = Some (range . end . saturating_sub (1)) ; Self :: raw (start_inclusive , end_inclusive) } }
};
}
