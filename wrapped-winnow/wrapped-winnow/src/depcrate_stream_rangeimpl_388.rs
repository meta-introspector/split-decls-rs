// Generated macro for impl_388 (impl)
macro_rules! Depcrate_stream_rangeimpl_388 {
() => {
// Module: crate::stream::range
// Provides: {"impl_388"}
// Dependencies: {}
impl core :: ops :: RangeBounds < usize > for Range { # [inline (always)] fn start_bound (& self) -> core :: ops :: Bound < & usize > { core :: ops :: Bound :: Included (& self . start_inclusive) } # [inline (always)] fn end_bound (& self) -> core :: ops :: Bound < & usize > { if let Some (end_inclusive) = & self . end_inclusive { core :: ops :: Bound :: Included (end_inclusive) } else { core :: ops :: Bound :: Unbounded } } }
};
}
