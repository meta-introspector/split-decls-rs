// Generated macro for impl_8 (impl)
macro_rules! Depcrate_subscriptionimpl_8 {
() => {
// Module: crate::subscription
// Provides: {"impl_8"}
// Dependencies: {}
impl < E > GraphQLSubscription < E , DefaultOnConnInitType , DefaultOnPingType > where E : Executor , { # [doc = " Create a [`GraphQLSubscription`] object."] pub fn new (executor : E) -> Self { GraphQLSubscription { executor , on_connection_init : default_on_connection_init , on_ping : default_on_ping , keepalive_timeout : None , } } }
};
}
