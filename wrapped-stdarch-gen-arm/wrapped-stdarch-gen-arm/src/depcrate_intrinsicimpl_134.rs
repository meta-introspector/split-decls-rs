// Generated macro for impl_134 (impl)
macro_rules! Depcrate_intrinsicimpl_134 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_134"}
// Dependencies: {}
impl fmt :: Display for Argument { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let AccessLevel :: RW = & self . rw { write ! (f , "mut ") ? ; } write ! (f , "{}: {}" , self . name , self . kind) } }
};
}
