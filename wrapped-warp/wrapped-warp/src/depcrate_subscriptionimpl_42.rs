// Generated macro for impl_42 (impl)
macro_rules! Depcrate_subscriptionimpl_42 {
() => {
// Module: crate::subscription
// Provides: {"impl_42"}
// Dependencies: {}
impl < Sink , Stream , E > GraphQLWebSocket < Sink , Stream , E , DefaultOnConnInitType , DefaultOnPingType > where Sink : futures_util :: sink :: Sink < Message > , Stream : futures_util :: stream :: Stream < Item = Result < Message , Error > > , E : Executor , { # [doc = " Create a [`GraphQLWebSocket`] object with sink and stream objects."] pub fn new_with_pair (sink : Sink , stream : Stream , executor : E , protocol : WebSocketProtocols ,) -> Self { GraphQLWebSocket { sink , stream , protocol , executor , data : Data :: default () , on_init : default_on_connection_init , on_ping : default_on_ping , keepalive_timeout : None , } } }
};
}
