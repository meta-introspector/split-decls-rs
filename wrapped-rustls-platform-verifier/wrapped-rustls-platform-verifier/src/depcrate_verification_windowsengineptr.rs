// Generated macro for EnginePtr (trait)
macro_rules! Depcrate_verification_windowsEnginePtr {
() => {
// Module: crate::verification::windows
// Provides: {"EnginePtr"}
// Dependencies: {}
# [doc = " An abstraction trait over the different ways various `windows-sys` versions represent"] # [doc = " the type of `HCERTCHAINENGINE`."] trait EnginePtr : Sized { fn from_raw (val : NonNull < c_void >) -> Self ; const NULL : Self ; }
};
}
