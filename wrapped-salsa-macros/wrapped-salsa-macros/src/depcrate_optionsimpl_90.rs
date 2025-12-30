// Generated macro for impl_90 (impl)
macro_rules! Depcrate_optionsimpl_90 {
() => {
// Module: crate::options
// Provides: {"impl_90"}
// Dependencies: {}
impl AllowedPersistOptions { fn allowed (& self) -> bool { matches ! (self , Self :: AllowedIdent | Self :: AllowedValue) } fn allowed_value (& self) -> bool { matches ! (self , Self :: AllowedValue) } }
};
}
