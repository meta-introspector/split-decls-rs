// Generated macro for impl_820 (impl)
macro_rules! Depcrate_cors_allow_private_networkimpl_820 {
() => {
// Module: crate::cors::allow_private_network
// Provides: {"impl_820"}
// Dependencies: {}
impl fmt :: Debug for AllowPrivateNetwork { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . 0 { AllowPrivateNetworkInner :: Yes => f . debug_tuple ("Yes") . finish () , AllowPrivateNetworkInner :: No => f . debug_tuple ("No") . finish () , AllowPrivateNetworkInner :: Predicate (_) => f . debug_tuple ("Predicate") . finish () , } } }
};
}
