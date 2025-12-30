// Generated macro for impl_632 (impl)
macro_rules! Depcrate_filters_wsimpl_632 {
() => {
// Module: crate::filters::ws
// Provides: {"impl_632"}
// Dependencies: {}
impl < F , U > Reply for WsReply < F > where F : FnOnce (WebSocket) -> U + Send + 'static , U : Future < Output = () > + Send + 'static , { fn into_response (self) -> Response { if let Some (on_upgrade) = self . ws . on_upgrade { let on_upgrade_cb = self . on_upgrade ; let config = self . ws . config ; let fut = on_upgrade . and_then (move | upgraded | { tracing :: trace ! ("websocket upgrade complete") ; WebSocket :: from_raw_socket (upgraded , protocol :: Role :: Server , config) . map (Ok) }) . and_then (move | socket | on_upgrade_cb (socket) . map (Ok)) . map (| result | { if let Err (err) = result { tracing :: debug ! ("ws upgrade error: {}" , err) ; } }) ; :: tokio :: task :: spawn (fut) ; } else { tracing :: debug ! ("ws couldn't be upgraded since no upgrade state was present") ; } let mut res = http :: Response :: default () ; * res . status_mut () = http :: StatusCode :: SWITCHING_PROTOCOLS ; res . headers_mut () . typed_insert (Connection :: upgrade ()) ; res . headers_mut () . typed_insert (Upgrade :: websocket ()) ; res . headers_mut () . typed_insert (SecWebsocketAccept :: from (self . ws . key)) ; res } }
};
}
