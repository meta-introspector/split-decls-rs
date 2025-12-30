// Generated macro for impl_301 (impl)
macro_rules! Depcrate_loom_std_unsafe_cellimpl_301 {
() => {
// Module: crate::loom::std::unsafe_cell
// Provides: {"impl_301"}
// Dependencies: {}
impl < T > UnsafeCell < T > { pub (crate) const fn new (data : T) -> UnsafeCell < T > { UnsafeCell (std :: cell :: UnsafeCell :: new (data)) } # [inline (always)] pub (crate) fn with < R > (& self , f : impl FnOnce (* const T) -> R) -> R { f (self . 0 . get ()) } # [inline (always)] pub (crate) fn with_mut < R > (& self , f : impl FnOnce (* mut T) -> R) -> R { f (self . 0 . get ()) } }
};
}
