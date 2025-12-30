// Generated macro for impl_44 (impl)
macro_rules! Depcrate_aes_ctrimpl_44 {
() => {
// Module: crate::aes_ctr
// Provides: {"impl_44"}
// Dependencies: {}
impl < C > fmt :: Debug for AesCtrZipKeyStream < C > where C : AesKind , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "AesCtrZipKeyStream<{}>(counter: {})" , any :: type_name ::< C > () , self . counter) } }
};
}
