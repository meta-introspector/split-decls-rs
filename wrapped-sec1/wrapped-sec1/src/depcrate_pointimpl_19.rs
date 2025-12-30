// Generated macro for impl_19 (impl)
macro_rules! Depcrate_pointimpl_19 {
() => {
// Module: crate::point
// Provides: {"impl_19"}
// Dependencies: {}
impl < Size > Debug for EncodedPoint < Size > where Size : ModulusSize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "EncodedPoint({:?})" , self . coordinates ()) } }
};
}
