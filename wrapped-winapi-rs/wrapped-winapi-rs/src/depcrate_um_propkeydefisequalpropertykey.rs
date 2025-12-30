// Generated macro for IsEqualPropertyKey (function)
macro_rules! Depcrate_um_propkeydefIsEqualPropertyKey {
() => {
// Module: crate::um::propkeydef
// Provides: {"IsEqualPropertyKey"}
// Dependencies: {}
# [inline] pub fn IsEqualPropertyKey (a : & PROPERTYKEY , b : & PROPERTYKEY) -> bool { (a . pid == b . pid) && IsEqualIID (& a . fmtid , & b . fmtid) }
};
}
