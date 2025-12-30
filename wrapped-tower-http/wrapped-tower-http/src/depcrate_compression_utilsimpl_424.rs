// Generated macro for impl_424 (impl)
macro_rules! Depcrate_compression_utilsimpl_424 {
() => {
// Module: crate::compression_utils
// Provides: {"impl_424"}
// Dependencies: {}
impl < S , T , E > Stream for StreamErrorIntoIoError < S , E > where S : Stream < Item = Result < T , E > > , { type Item = Result < T , io :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match ready ! (this . inner . poll_next (cx)) { None => Poll :: Ready (None) , Some (Ok (value)) => Poll :: Ready (Some (Ok (value))) , Some (Err (err)) => { * this . error = Some (err) ; Poll :: Ready (Some (Err (io :: Error :: from_raw_os_error (SENTINEL_ERROR_CODE)))) } } } }
};
}
