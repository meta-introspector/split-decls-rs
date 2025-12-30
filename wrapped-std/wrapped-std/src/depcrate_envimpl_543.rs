// Generated macro for impl_543 (impl)
macro_rules! Depcrate_envimpl_543 {
() => {
// Module: crate::env
// Provides: {"impl_543"}
// Dependencies: {}
# [stable (feature = "env" , since = "1.0.0")] impl fmt :: Display for VarError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { VarError :: NotPresent => write ! (f , "environment variable not found") , VarError :: NotUnicode (ref s) => { write ! (f , "environment variable was not valid unicode: {:?}" , s) } } } }
};
}
