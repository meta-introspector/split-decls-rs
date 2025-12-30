// Generated macro for impl_45 (impl)
macro_rules! Depcrate_errorimpl_45 {
() => {
// Module: crate::error
// Provides: {"impl_45"}
// Dependencies: {}
impl fmt :: Display for PathError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} at path {:?}" , self . err , self . path) } }
};
}
