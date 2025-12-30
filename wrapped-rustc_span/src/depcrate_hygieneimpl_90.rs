// Generated macro for impl_90 (impl)
macro_rules! Depcrate_hygieneimpl_90 {
() => {
// Module: crate::hygiene
// Provides: {"impl_90"}
// Dependencies: {}
impl fmt :: Debug for ExpnId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:?}::{{{{expn{}}}}}" , self . krate , self . local_id . as_u32 ()) } }
};
}
