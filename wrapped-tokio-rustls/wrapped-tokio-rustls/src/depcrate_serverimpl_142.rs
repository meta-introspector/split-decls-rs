// Generated macro for impl_142 (impl)
macro_rules! Depcrate_serverimpl_142 {
() => {
// Module: crate::server
// Provides: {"impl_142"}
// Dependencies: {}
impl < IO > AsyncBufRead for TlsStream < IO > where IO : AsyncRead + AsyncWrite + Unpin , { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { match self . state { TlsState :: Stream | TlsState :: WriteShutdown => { let this = self . get_mut () ; let stream = Stream :: new (& mut this . io , & mut this . session) . set_eof (! this . state . readable ()) ; match stream . poll_fill_buf (cx) { Poll :: Ready (Ok (buf)) => { if buf . is_empty () { this . state . shutdown_read () ; } Poll :: Ready (Ok (buf)) } Poll :: Ready (Err (err)) if err . kind () == io :: ErrorKind :: ConnectionAborted => { this . state . shutdown_read () ; Poll :: Ready (Err (err)) } output => output , } } TlsState :: ReadShutdown | TlsState :: FullyShutdown => Poll :: Ready (Ok (& [])) , # [cfg (feature = "early-data")] ref s => unreachable ! ("server TLS can not hit this state: {:?}" , s) , } } fn consume (mut self : Pin < & mut Self > , amt : usize) { self . session . reader () . consume (amt) ; } }
};
}
