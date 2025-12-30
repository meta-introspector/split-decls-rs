// Generated macro for impl_88 (impl)
macro_rules! Depcrate_common_test_streamimpl_88 {
() => {
// Module: crate::common::test_stream
// Provides: {"impl_88"}
// Dependencies: {}
impl AsyncWrite for Good < '_ > { fn poll_write (mut self : Pin < & mut Self > , _cx : & mut Context < '_ > , mut buf : & [u8] ,) -> Poll < io :: Result < usize > > { let len = self . 0 . read_tls (buf . by_ref ()) ? ; self . 0 . process_new_packets () . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err)) ? ; Poll :: Ready (Ok (len)) } fn poll_flush (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . 0 . process_new_packets () . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err)) ? ; Poll :: Ready (Ok (())) } fn poll_shutdown (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . 0 . send_close_notify () ; dbg ! ("sent close notify") ; self . poll_flush (cx) } }
};
}
