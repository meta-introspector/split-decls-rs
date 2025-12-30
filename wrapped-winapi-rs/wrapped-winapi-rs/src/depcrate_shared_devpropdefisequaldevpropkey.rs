// Generated macro for IsEqualDevPropKey (function)
macro_rules! Depcrate_shared_devpropdefIsEqualDevPropKey {
() => {
// Module: crate::shared::devpropdef
// Provides: {"IsEqualDevPropKey"}
// Dependencies: {}
# [inline] pub fn IsEqualDevPropKey (a : & DEVPROPKEY , b : & DEVPROPKEY) -> bool { (a . pid == b . pid) && IsEqualGUID (& a . fmtid , & b . fmtid) }
};
}
