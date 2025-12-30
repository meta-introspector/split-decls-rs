// Generated macro for impl_89 (impl)
macro_rules! Depcrate_patimpl_89 {
() => {
// Module: crate::pat
// Provides: {"impl_89"}
// Dependencies: {}
impl < 'p , Cx : PatCx > fmt :: Debug for PatOrWild < 'p , Cx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { PatOrWild :: Wild => write ! (f , "_") , PatOrWild :: Pat (pat) => pat . fmt (f) , } } }
};
}
