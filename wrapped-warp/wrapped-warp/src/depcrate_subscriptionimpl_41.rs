// Generated macro for impl_41 (impl)
macro_rules! Depcrate_subscriptionimpl_41 {
() => {
// Module: crate::subscription
// Provides: {"impl_41"}
// Dependencies: {}
impl < S , E > GraphQLWebSocket < SplitSink < S , Message > , SplitStream < S > , E , DefaultOnConnInitType , DefaultOnPingType , > where S : Stream < Item = Result < Message , Error > > + Sink < Message > , E : Executor , { # [doc = " Create a [`GraphQLWebSocket`] object."] pub fn new (socket : S , executor : E , protocol : WebSocketProtocols) -> Self { let (sink , stream) = socket . split () ; GraphQLWebSocket :: new_with_pair (sink , stream , executor , protocol) } }
};
}
