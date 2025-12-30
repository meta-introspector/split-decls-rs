// Generated macro for WsClient (struct)
macro_rules! Depcrate_testWsClient {
() => {
// Module: crate::test
// Provides: {"WsClient"}
// Dependencies: {}
# [doc = " A test client for Websocket filters."] # [cfg (feature = "websocket")] pub struct WsClient { tx : mpsc :: UnboundedSender < crate :: ws :: Message > , rx : mpsc :: UnboundedReceiver < Result < crate :: ws :: Message , crate :: error :: Error > > , }
};
}
