// Generated macro for impl_617 (impl)
macro_rules! Depcrate_ule_multiimpl_617 {
() => {
// Module: crate::ule::multi
// Provides: {"impl_617"}
// Dependencies: {}
impl < const LEN : usize , Format : VarZeroVecFormat > fmt :: Debug for MultiFieldsULE < LEN , Format > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "MultiFieldsULE<{LEN}>({:?})" , self . 0 . as_bytes ()) } }
};
}
