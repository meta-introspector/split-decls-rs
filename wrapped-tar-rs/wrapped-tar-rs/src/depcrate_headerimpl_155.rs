// Generated macro for impl_155 (impl)
macro_rules! Depcrate_headerimpl_155 {
() => {
// Module: crate::header
// Provides: {"impl_155"}
// Dependencies: {}
impl < 'a > fmt :: Debug for DebugSparseHeaders < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut f = f . debug_list () ; for header in self . 0 { if ! header . is_empty () { f . entry (header) ; } } f . finish () } }
};
}
