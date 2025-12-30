// Generated macro for impl_157 (impl)
macro_rules! Depcrate_headerimpl_157 {
() => {
// Module: crate::header
// Provides: {"impl_157"}
// Dependencies: {}
impl fmt :: Debug for GnuSparseHeader { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut f = f . debug_struct ("GnuSparseHeader") ; if let Ok (offset) = self . offset () { f . field ("offset" , & offset) ; } if let Ok (length) = self . length () { f . field ("length" , & length) ; } f . finish () } }
};
}
