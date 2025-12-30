// Generated macro for impl_534 (impl)
macro_rules! Depcrate_predicateimpl_534 {
() => {
// Module: crate::predicate
// Provides: {"impl_534"}
// Dependencies: {}
impl fmt :: Display for ImplPolarity { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Positive => f . write_str ("positive") , Self :: Negative => f . write_str ("negative") , Self :: Reservation => f . write_str ("reservation") , } } }
};
}
