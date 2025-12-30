// Generated macro for impl_769 (impl)
macro_rules! Depcrate_cors_allow_credentialsimpl_769 {
() => {
// Module: crate::cors::allow_credentials
// Provides: {"impl_769"}
// Dependencies: {}
impl fmt :: Debug for AllowCredentials { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . 0 { AllowCredentialsInner :: Yes => f . debug_tuple ("Yes") . finish () , AllowCredentialsInner :: No => f . debug_tuple ("No") . finish () , AllowCredentialsInner :: Predicate (_) => f . debug_tuple ("Predicate") . finish () , } } }
};
}
