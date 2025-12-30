// Generated macro for BCRYPT_MAKE_INTERFACE_VERSION (macro)
macro_rules! Depcrate_macrosBCRYPT_MAKE_INTERFACE_VERSION {
() => {
// Module: crate::macros
// Provides: {"BCRYPT_MAKE_INTERFACE_VERSION"}
// Dependencies: {}
macro_rules ! BCRYPT_MAKE_INTERFACE_VERSION { ($ major : expr , $ minor : expr) => { $ crate :: shared :: bcrypt :: BCRYPT_INTERFACE_VERSION { MajorVersion : $ major , MinorVersion : $ minor , } } }
};
}
