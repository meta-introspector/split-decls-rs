// Generated macro for impl_844 (impl)
macro_rules! Depcrate_testimpl_844 {
() => {
// Module: crate::test
// Provides: {"impl_844"}
// Dependencies: {}
# [cfg (feature = "websocket")] impl Stream for WsClient { type Item = Result < crate :: ws :: Message , WsError > ; fn poll_next (self : Pin < & mut Self > , context : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = Pin :: into_inner (self) ; let rx = Pin :: new (& mut this . rx) ; match rx . poll_next (context) { Poll :: Ready (Some (result)) => Poll :: Ready (Some (result . map_err (WsError :: new))) , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Pending => Poll :: Pending , } } }
};
}
