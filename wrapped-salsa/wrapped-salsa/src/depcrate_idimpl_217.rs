// Generated macro for impl_217 (impl)
macro_rules! Depcrate_idimpl_217 {
() => {
// Module: crate::id
// Provides: {"impl_217"}
// Dependencies: {}
impl Debug for Id { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if self . generation () == 0 { write ! (f , "Id({:x})" , self . index ()) } else { write ! (f , "Id({:x}g{:x})" , self . index () , self . generation ()) } } }
};
}
