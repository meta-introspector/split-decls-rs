// Generated macro for impl_103 (impl)
macro_rules! Depcrate_databakeimpl_103 {
() => {
// Module: crate::databake
// Provides: {"impl_103"}
// Dependencies: {}
impl < const N : usize > databake :: Bake for UnvalidatedTinyAsciiStr < N > { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { match self . try_into_tinystr () { Ok (tiny) => { let tiny = tiny . bake (env) ; databake :: quote ! { # tiny . to_unvalidated () } } Err (_) => { let bytes = self . 0 . bake (env) ; env . insert ("tinystr") ; databake :: quote ! { tinystr :: UnvalidatedTinyAsciiStr :: from_utf8_unchecked (# bytes) } } } } }
};
}
