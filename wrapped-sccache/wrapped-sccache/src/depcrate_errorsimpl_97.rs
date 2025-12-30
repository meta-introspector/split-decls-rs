// Generated macro for impl_97 (impl)
macro_rules! Depcrate_errorsimpl_97 {
() => {
// Module: crate::errors
// Provides: {"impl_97"}
// Dependencies: {}
impl std :: fmt :: Display for ProcessError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , String :: from_utf8_lossy (& self . 0 . stderr)) } }
};
}
