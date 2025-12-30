// Generated macro for impl_float_approxeq (macro)
macro_rules! Depcrate_approxeqimpl_float_approxeq {
() => {
// Module: crate::approxeq
// Provides: {"impl_float_approxeq"}
// Dependencies: {}
macro_rules ! impl_float_approxeq { { $ ($ type : ty) ,* } => { $ (impl ApproxEq for $ type { fn approxeq (& self , other : & Self , ulps : i64) -> bool { if self . is_nan () && other . is_nan () { true } else { (self . ulps (other) as i64) . abs () <= ulps } } fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{:?} ({:x})" , self , self . to_bits ()) } }) * } ; }
};
}
