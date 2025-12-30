// Generated macro for impl_11 (impl)
macro_rules! Depcrate_errorimpl_11 {
() => {
// Module: crate::error
// Provides: {"impl_11"}
// Dependencies: {}
impl From < SystemError > for ProgramError { fn from (e : SystemError) -> Self { Self :: Custom (e as u32) } }
};
}
