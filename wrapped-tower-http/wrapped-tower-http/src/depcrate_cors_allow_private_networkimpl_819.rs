// Generated macro for impl_819 (impl)
macro_rules! Depcrate_cors_allow_private_networkimpl_819 {
() => {
// Module: crate::cors::allow_private_network
// Provides: {"impl_819"}
// Dependencies: {}
impl From < bool > for AllowPrivateNetwork { fn from (v : bool) -> Self { match v { true => Self (AllowPrivateNetworkInner :: Yes) , false => Self (AllowPrivateNetworkInner :: No) , } } }
};
}
