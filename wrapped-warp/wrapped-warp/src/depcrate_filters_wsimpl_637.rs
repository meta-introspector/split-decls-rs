// Generated macro for impl_637 (impl)
macro_rules! Depcrate_filters_wsimpl_637 {
() => {
// Module: crate::filters::ws
// Provides: {"impl_637"}
// Dependencies: {}
impl Sink < Message > for WebSocket { type Error = crate :: Error ; fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match ready ! (Pin :: new (& mut self . inner) . poll_ready (cx)) { Ok (()) => Poll :: Ready (Ok (())) , Err (e) => Poll :: Ready (Err (crate :: Error :: new (e))) , } } fn start_send (mut self : Pin < & mut Self > , item : Message) -> Result < () , Self :: Error > { match Pin :: new (& mut self . inner) . start_send (item . inner) { Ok (()) => Ok (()) , Err (e) => { tracing :: debug ! ("websocket start_send error: {}" , e) ; Err (crate :: Error :: new (e)) } } } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match ready ! (Pin :: new (& mut self . inner) . poll_flush (cx)) { Ok (()) => Poll :: Ready (Ok (())) , Err (e) => Poll :: Ready (Err (crate :: Error :: new (e))) , } } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match ready ! (Pin :: new (& mut self . inner) . poll_close (cx)) { Ok (()) => Poll :: Ready (Ok (())) , Err (err) => { tracing :: debug ! ("websocket close error: {}" , err) ; Poll :: Ready (Err (crate :: Error :: new (err))) } } } }
};
}
