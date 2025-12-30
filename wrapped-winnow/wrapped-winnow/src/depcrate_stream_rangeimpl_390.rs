// Generated macro for impl_390 (impl)
macro_rules! Depcrate_stream_rangeimpl_390 {
() => {
// Module: crate::stream::range
// Provides: {"impl_390"}
// Dependencies: {}
impl From < core :: ops :: Range < usize > > for Range { # [inline (always)] fn from (range : core :: ops :: Range < usize >) -> Self { let start_inclusive = range . start ; let end_inclusive = Some (range . end . saturating_sub (1)) ; Self :: raw (start_inclusive , end_inclusive) } }
};
}
