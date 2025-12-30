// Generated macro for impl_117 (impl)
macro_rules! Depcrateimpl_117 {
() => {
// Module: crate
// Provides: {"impl_117"}
// Dependencies: {}
impl BackgroundTask { fn new (loki_url : Url , http_headers : reqwest :: header :: HeaderMap , receiver : mpsc :: Receiver < Option < LokiEvent > > , labels : & FormattedLabels ,) -> Result < BackgroundTask , Error > { Ok (BackgroundTask { receiver , loki_url : loki_url . join ("loki/api/v1/push") . map_err (| _ | Error (ErrorI :: InvalidLokiUrl)) ? , queues : LevelMap :: from_fn (| level | SendQueue :: new (labels . finish (level))) , buffer : Buffer :: new () , http_client : reqwest :: Client :: builder () . user_agent (concat ! (env ! ("CARGO_PKG_NAME") , "/" , env ! ("CARGO_PKG_VERSION"))) . default_headers (http_headers) . redirect (reqwest :: redirect :: Policy :: custom (| a | { let status = a . status () . as_u16 () ; if status == 302 || status == 303 { let to = a . url () . clone () ; return a . error (BadRedirect { status , to }) ; } reqwest :: redirect :: Policy :: default () . redirect (a) })) . build () . expect ("reqwest client builder") , backoff_count : 0 , backoff : None , quitting : false , send_task : None , }) } fn backoff_time (& self) -> (bool , Duration) { let backoff_time = if self . backoff_count >= 1 { Duration :: from_millis (500u64 . checked_shl (self . backoff_count - 1) . unwrap_or (u64 :: MAX) ,) } else { Duration :: from_millis (0) } ; (backoff_time >= Duration :: from_secs (30) , cmp :: min (backoff_time , Duration :: from_secs (600)) ,) } }
};
}
