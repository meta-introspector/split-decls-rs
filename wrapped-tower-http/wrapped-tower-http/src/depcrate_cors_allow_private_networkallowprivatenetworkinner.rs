// Generated macro for AllowPrivateNetworkInner (enum)
macro_rules! Depcrate_cors_allow_private_networkAllowPrivateNetworkInner {
() => {
// Module: crate::cors::allow_private_network
// Provides: {"AllowPrivateNetworkInner"}
// Dependencies: {}
# [derive (Clone)] enum AllowPrivateNetworkInner { Yes , No , Predicate (Arc < dyn for < 'a > Fn (& 'a HeaderValue , & 'a RequestParts) -> bool + Send + Sync + 'static > ,) , }
};
}
