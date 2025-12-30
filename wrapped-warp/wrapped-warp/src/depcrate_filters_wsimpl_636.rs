// Generated macro for impl_636 (impl)
macro_rules! Depcrate_filters_wsimpl_636 {
() => {
// Module: crate::filters::ws
// Provides: {"impl_636"}
// Dependencies: {}
impl Stream for WebSocket { type Item = Result < Message , crate :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match ready ! (Pin :: new (& mut self . inner) . poll_next (cx)) { Some (Ok (item)) => Poll :: Ready (Some (Ok (Message { inner : item }))) , Some (Err (e)) => { tracing :: debug ! ("websocket poll error: {}" , e) ; Poll :: Ready (Some (Err (crate :: Error :: new (e)))) } None => { tracing :: trace ! ("websocket closed") ; Poll :: Ready (None) } } } }
};
}
