// Generated macro for impl_841 (impl)
macro_rules! Depcrate_cors_max_ageimpl_841 {
() => {
// Module: crate::cors::max_age
// Provides: {"impl_841"}
// Dependencies: {}
impl fmt :: Debug for MaxAge { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . 0 { MaxAgeInner :: Exact (inner) => f . debug_tuple ("Exact") . field (inner) . finish () , MaxAgeInner :: Fn (_) => f . debug_tuple ("Fn") . finish () , } } }
};
}
