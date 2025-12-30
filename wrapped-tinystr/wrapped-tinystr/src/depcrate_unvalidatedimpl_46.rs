// Generated macro for impl_46 (impl)
macro_rules! Depcrate_unvalidatedimpl_46 {
() => {
// Module: crate::unvalidated
// Provides: {"impl_46"}
// Dependencies: {}
impl < const N : usize > fmt :: Debug for UnvalidatedTinyAsciiStr < N > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_into_tinystr () { Ok (s) => fmt :: Debug :: fmt (& s , f) , Err (_) => fmt :: Debug :: fmt (& self . 0 , f) , } } }
};
}
