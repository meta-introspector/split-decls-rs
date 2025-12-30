// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
impl fmt :: Debug for ClientStorage { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "(ops: {:?})" , self . ops . lock () . unwrap ()) } }
};
}
