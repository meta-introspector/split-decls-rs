// Generated macro for impl_310 (impl)
macro_rules! Depcrate_fatal_errorimpl_310 {
() => {
// Module: crate::fatal_error
// Provides: {"impl_310"}
// Dependencies: {}
impl FatalError { pub fn raise (self) -> ! { std :: panic :: resume_unwind (Box :: new (FatalErrorMarker)) } }
};
}
