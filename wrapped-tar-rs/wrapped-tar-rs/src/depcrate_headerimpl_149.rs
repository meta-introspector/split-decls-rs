// Generated macro for impl_149 (impl)
macro_rules! Depcrate_headerimpl_149 {
() => {
// Module: crate::header
// Provides: {"impl_149"}
// Dependencies: {}
impl fmt :: Debug for OldHeader { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut f = f . debug_struct ("OldHeader") ; self . as_header () . debug_fields (& mut f) ; f . finish () } }
};
}
