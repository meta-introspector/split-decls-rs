// Generated macro for impl_635 (impl)
macro_rules! Depcrate_filters_wsimpl_635 {
() => {
// Module: crate::filters::ws
// Provides: {"impl_635"}
// Dependencies: {}
impl WebSocket { pub (crate) async fn from_raw_socket (upgraded : hyper :: upgrade :: Upgraded , role : protocol :: Role , config : Option < protocol :: WebSocketConfig > ,) -> Self { let upgraded = hyper_util :: rt :: TokioIo :: new (upgraded) ; WebSocketStream :: from_raw_socket (upgraded , role , config) . map (| inner | WebSocket { inner }) . await } # [doc = " Gracefully close this websocket."] pub async fn close (mut self) -> Result < () , crate :: Error > { future :: poll_fn (| cx | Pin :: new (& mut self) . poll_close (cx)) . await } }
};
}
