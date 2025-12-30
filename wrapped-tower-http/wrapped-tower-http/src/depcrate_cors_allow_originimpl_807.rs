// Generated macro for impl_807 (impl)
macro_rules! Depcrate_cors_allow_originimpl_807 {
() => {
// Module: crate::cors::allow_origin
// Provides: {"impl_807"}
// Dependencies: {}
impl fmt :: Debug for AllowOrigin { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . 0 { OriginInner :: Const (inner) => f . debug_tuple ("Const") . field (inner) . finish () , OriginInner :: List (inner) => f . debug_tuple ("List") . field (inner) . finish () , OriginInner :: Predicate (_) => f . debug_tuple ("Predicate") . finish () , OriginInner :: AsyncPredicate (_) => f . debug_tuple ("AsyncPredicate") . finish () , } } }
};
}
