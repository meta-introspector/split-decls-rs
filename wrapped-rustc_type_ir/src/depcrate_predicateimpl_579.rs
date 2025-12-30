// Generated macro for impl_579 (impl)
macro_rules! Depcrate_predicateimpl_579 {
() => {
// Module: crate::predicate
// Provides: {"impl_579"}
// Dependencies: {}
impl fmt :: Display for BoundConstness { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Const => f . write_str ("const") , Self :: Maybe => f . write_str ("[const]") , } } }
};
}
