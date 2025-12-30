// Generated macro for impl_117 (impl)
macro_rules! Depcrate_errorimpl_117 {
() => {
// Module: crate::error
// Provides: {"impl_117"}
// Dependencies: {}
impl Error { pub fn already_printed (& self) -> bool { use self :: Error :: * ; matches ! (self , CargoFail | Mismatch | RunFailed | ShouldNotHaveCompiled) } }
};
}
