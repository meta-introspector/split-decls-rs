// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl RustStringInner { fn as_opaque (& self) -> & RustString { let ptr : * const RustStringInner = ptr :: from_ref (self) ; let ptr = ptr as * const RustString ; unsafe { & * ptr } } fn from_opaque (opaque : & RustString) -> & Self { let ptr : * const RustString = ptr :: from_ref (opaque) ; let ptr : * const RustStringInner = ptr . cast () ; unsafe { & * ptr } } fn into_inner (self) -> Vec < u8 > { self . bytes . into_inner () } }
};
}
