// Generated macro for impl_integer_biteq (macro)
macro_rules! Depcrate_biteqimpl_integer_biteq {
() => {
// Module: crate::biteq
// Provides: {"impl_integer_biteq"}
// Dependencies: {}
macro_rules ! impl_integer_biteq { { $ ($ type : ty) ,* } => { $ (impl BitEq for $ type { fn biteq (& self , other : & Self) -> bool { self == other } fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{:?} ({:x})" , self , self) } }) * } ; }
};
}
