// Generated macro for create_sealable (function)
macro_rules! Depcrate_memfdcreate_sealable {
() => {
// Module: crate::memfd
// Provides: {"create_sealable"}
// Dependencies: {}
pub fn create_sealable () -> Result < File > { create (MFD_ALLOW_SEALING | MFD_CLOEXEC) }
};
}
