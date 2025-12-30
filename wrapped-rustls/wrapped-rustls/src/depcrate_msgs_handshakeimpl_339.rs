// Generated macro for impl_339 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_339 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_339"}
// Dependencies: {}
impl ServerEcdhParams { pub (crate) fn new (kx : & dyn ActiveKeyExchange) -> Self { Self { curve_params : EcParameters { curve_type : ECCurveType :: NamedCurve , named_group : kx . group () , } , public : PayloadU8 :: new (kx . pub_key () . to_vec ()) , } } }
};
}
