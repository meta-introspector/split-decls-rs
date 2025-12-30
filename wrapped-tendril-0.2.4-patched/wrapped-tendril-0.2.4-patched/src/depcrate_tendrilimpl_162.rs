// Generated macro for impl_162 (impl)
macro_rules! Depcrate_tendrilimpl_162 {
() => {
// Module: crate::tendril
// Provides: {"impl_162"}
// Dependencies: {}
impl < A > strfmt :: Write for Tendril < fmt :: UTF8 , A > where A : Atomicity , { # [inline] fn write_str (& mut self , s : & str) -> strfmt :: Result { self . push_slice (s) ; Ok (()) } }
};
}
