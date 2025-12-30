// Generated macro for impl_71 (impl)
macro_rules! Depcrate_diffimpl_71 {
() => {
// Module: crate::diff
// Provides: {"impl_71"}
// Dependencies: {}
impl fmt :: Debug for SignedDuration { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_positive { write ! (f , "+") ? ; } else { write ! (f , "-") ? ; } write ! (f , "{:?}" , self . duration) } }
};
}
