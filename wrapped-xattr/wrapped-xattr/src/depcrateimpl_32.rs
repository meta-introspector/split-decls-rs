// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl fmt :: Debug for XAttrs { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct AsList < 'a > (& 'a XAttrs) ; impl < 'a > fmt :: Debug for AsList < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . 0 . clone ()) . finish () } } f . debug_tuple ("XAttrs") . field (& AsList (self)) . finish () } }
};
}
