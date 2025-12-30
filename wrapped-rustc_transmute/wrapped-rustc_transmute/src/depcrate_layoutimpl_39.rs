// Generated macro for impl_39 (impl)
macro_rules! Depcrate_layoutimpl_39 {
() => {
// Module: crate::layout
// Provides: {"impl_39"}
// Dependencies: {}
impl fmt :: Debug for Byte { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . start == Self :: UNINIT && self . end == Self :: UNINIT + 1 { write ! (f , "uninit") } else if self . start <= Self :: UNINIT && self . end == Self :: UNINIT + 1 { write ! (f , "{}..{}|uninit" , self . start , self . end - 1) } else { write ! (f , "{}..{}" , self . start , self . end) } } }
};
}
