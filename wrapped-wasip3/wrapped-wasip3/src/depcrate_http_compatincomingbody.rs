// Generated macro for IncomingBody (struct)
macro_rules! Depcrate_http_compatIncomingBody {
() => {
// Module: crate::http_compat
// Provides: {"IncomingBody"}
// Dependencies: {}
# [doc = " A stream of Bytes, used when receiving bodies from the network."] pub struct IncomingBody < T > { state : StartedState < T > , content_length : Option < u64 > , }
};
}
