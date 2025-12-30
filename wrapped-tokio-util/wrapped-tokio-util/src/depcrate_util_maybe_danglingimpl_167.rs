// Generated macro for impl_167 (impl)
macro_rules! Depcrate_util_maybe_danglingimpl_167 {
() => {
// Module: crate::util::maybe_dangling
// Provides: {"impl_167"}
// Dependencies: {}
impl < T > Drop for MaybeDangling < T > { fn drop (& mut self) { unsafe { core :: ptr :: drop_in_place (self . 0 . as_mut_ptr ()) } ; } }
};
}
