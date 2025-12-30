// Generated macro for CTL_CODE (macro)
macro_rules! Depcrate_macrosCTL_CODE {
() => {
// Module: crate::macros
// Provides: {"CTL_CODE"}
// Dependencies: {}
macro_rules ! CTL_CODE { ($ DeviceType : expr , $ Function : expr , $ Method : expr , $ Access : expr) => { ($ DeviceType << 16) | ($ Access << 14) | ($ Function << 2) | $ Method } }
};
}
