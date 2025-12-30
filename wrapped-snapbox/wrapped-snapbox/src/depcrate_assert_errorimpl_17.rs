// Generated macro for impl_17 (impl)
macro_rules! Depcrate_assert_errorimpl_17 {
() => {
// Module: crate::assert::error
// Provides: {"impl_17"}
// Dependencies: {}
impl Error { pub fn new (inner : impl std :: fmt :: Display) -> Self { Self :: with_string (inner . to_string ()) } fn with_string (inner : String) -> Self { Self { inner , backtrace : Backtrace :: new () , } } # [track_caller] pub (crate) fn panic (self) -> ! { panic ! ("{self}") } }
};
}
