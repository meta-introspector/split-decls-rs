// Generated macro for impl_790 (impl)
macro_rules! Depcrate_cors_allow_methodsimpl_790 {
() => {
// Module: crate::cors::allow_methods
// Provides: {"impl_790"}
// Dependencies: {}
impl fmt :: Debug for AllowMethods { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . 0 { AllowMethodsInner :: Const (inner) => f . debug_tuple ("Const") . field (inner) . finish () , AllowMethodsInner :: MirrorRequest => f . debug_tuple ("MirrorRequest") . finish () , } } }
};
}
