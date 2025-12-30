// Generated macro for impl_147 (impl)
macro_rules! Depcrate_headerimpl_147 {
() => {
// Module: crate::header
// Provides: {"impl_147"}
// Dependencies: {}
impl fmt :: Debug for Header { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if let Some (me) = self . as_ustar () { me . fmt (f) } else if let Some (me) = self . as_gnu () { me . fmt (f) } else { self . as_old () . fmt (f) } } }
};
}
