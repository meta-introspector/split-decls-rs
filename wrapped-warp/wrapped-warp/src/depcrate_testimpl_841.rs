// Generated macro for impl_841 (impl)
macro_rules! Depcrate_testimpl_841 {
() => {
// Module: crate::test
// Provides: {"impl_841"}
// Dependencies: {}
# [cfg (feature = "websocket")] impl WsClient { # [doc = " Send a \"text\" websocket message to the server."] pub async fn send_text (& mut self , text : impl Into < String >) { self . send (crate :: ws :: Message :: text (text . into ())) . await ; } # [doc = " Send a websocket message to the server."] pub async fn send (& mut self , msg : crate :: ws :: Message) { self . tx . unbounded_send (msg) . unwrap () ; } # [doc = " Receive a websocket message from the server."] pub async fn recv (& mut self) -> Result < crate :: filters :: ws :: Message , WsError > { self . rx . next () . await . map (| result | result . map_err (WsError :: new)) . unwrap_or_else (| | { Err (WsError :: new ("closed")) }) } # [doc = " Assert the server has closed the connection."] pub async fn recv_closed (& mut self) -> Result < () , WsError > { self . rx . next () . await . map (| result | match result { Ok (msg) => Err (WsError :: new (format ! ("received message: {:?}" , msg))) , Err (err) => Err (WsError :: new (err)) , }) . unwrap_or_else (| | { Ok (()) }) } fn pinned_tx (self : Pin < & mut Self >) -> Pin < & mut mpsc :: UnboundedSender < crate :: ws :: Message > > { let this = Pin :: into_inner (self) ; Pin :: new (& mut this . tx) } }
};
}
