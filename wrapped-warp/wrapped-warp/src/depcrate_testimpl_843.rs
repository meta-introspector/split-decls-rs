// Generated macro for impl_843 (impl)
macro_rules! Depcrate_testimpl_843 {
() => {
// Module: crate::test
// Provides: {"impl_843"}
// Dependencies: {}
# [cfg (feature = "websocket")] impl Sink < crate :: ws :: Message > for WsClient { type Error = WsError ; fn poll_ready (self : Pin < & mut Self > , context : & mut Context < '_ > ,) -> Poll < Result < () , Self :: Error > > { self . pinned_tx () . poll_ready (context) . map_err (WsError :: new) } fn start_send (self : Pin < & mut Self > , message : Message) -> Result < () , Self :: Error > { self . pinned_tx () . start_send (message) . map_err (WsError :: new) } fn poll_flush (self : Pin < & mut Self > , context : & mut Context < '_ > ,) -> Poll < Result < () , Self :: Error > > { self . pinned_tx () . poll_flush (context) . map_err (WsError :: new) } fn poll_close (self : Pin < & mut Self > , context : & mut Context < '_ > ,) -> Poll < Result < () , Self :: Error > > { self . pinned_tx () . poll_close (context) . map_err (WsError :: new) } }
};
}
