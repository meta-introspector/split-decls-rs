// Generated macro for impl_279 (impl)
macro_rules! Depcrate_typesimpl_279 {
() => {
// Module: crate::types
// Provides: {"impl_279"}
// Dependencies: {}
impl fmt :: Display for ChangeTag { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , match & self { ChangeTag :: Equal => ' ' , ChangeTag :: Delete => '-' , ChangeTag :: Insert => '+' , }) } }
};
}
