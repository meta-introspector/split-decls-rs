// Generated macro for WebSocket (struct)
macro_rules! Depcrate_filters_wsWebSocket {
() => {
// Module: crate::filters::ws
// Provides: {"WebSocket"}
// Dependencies: {}
# [doc = " A websocket `Stream` and `Sink`, provided to `ws` filters."] # [doc = ""] # [doc = " Ping messages sent from the client will be handled internally by replying with a Pong message."] # [doc = " Close messages need to be handled explicitly: usually by closing the `Sink` end of the"] # [doc = " `WebSocket`."] # [doc = ""] # [doc = " **Note!**"] # [doc = " Due to rust futures nature, pings won't be handled until read part of `WebSocket` is polled"] pub struct WebSocket { inner : WebSocketStream < hyper_util :: rt :: TokioIo < hyper :: upgrade :: Upgraded > > , }
};
}
