// Generated macro for impl_168 (impl)
macro_rules! Depcrate_util_maybe_danglingimpl_168 {
() => {
// Module: crate::util::maybe_dangling
// Provides: {"impl_168"}
// Dependencies: {}
impl < T > MaybeDangling < T > { pub (crate) fn new (inner : T) -> Self { Self (MaybeUninit :: new (inner)) } }
};
}
