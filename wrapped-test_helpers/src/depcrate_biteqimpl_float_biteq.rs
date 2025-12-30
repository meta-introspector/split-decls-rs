// Generated macro for impl_float_biteq (macro)
macro_rules! Depcrate_biteqimpl_float_biteq {
() => {
// Module: crate::biteq
// Provides: {"impl_float_biteq"}
// Dependencies: {}
macro_rules ! impl_float_biteq { { $ ($ type : ty) ,* } => { $ (impl BitEq for $ type { fn biteq (& self , other : & Self) -> bool { if self . is_nan () && other . is_nan () { true } else { self . to_bits () == other . to_bits () } } fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{:?} ({:x})" , self , self . to_bits ()) } }) * } ; }
};
}
