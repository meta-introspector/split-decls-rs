// Generated macro for impl_298 (impl)
macro_rules! Depcrate_fatal_errorimpl_298 {
() => {
// Module: crate::fatal_error
// Provides: {"impl_298"}
// Dependencies: {}
impl FatalError { pub fn raise (self) -> ! { std :: panic :: resume_unwind (Box :: new (FatalErrorMarker)) } }
};
}
