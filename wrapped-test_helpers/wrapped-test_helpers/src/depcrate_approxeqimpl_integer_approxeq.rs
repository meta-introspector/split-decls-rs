// Generated macro for impl_integer_approxeq (macro)
macro_rules! Depcrate_approxeqimpl_integer_approxeq {
() => {
// Module: crate::approxeq
// Provides: {"impl_integer_approxeq"}
// Dependencies: {}
macro_rules ! impl_integer_approxeq { { $ ($ type : ty) ,* } => { $ (impl ApproxEq for $ type { fn approxeq (& self , other : & Self , _ulps : i64) -> bool { self == other } fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{:?} ({:x})" , self , self) } }) * } ; }
};
}
