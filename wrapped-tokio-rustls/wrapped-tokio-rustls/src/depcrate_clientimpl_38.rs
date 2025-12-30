// Generated macro for impl_38 (impl)
macro_rules! Depcrate_clientimpl_38 {
() => {
// Module: crate::client
// Provides: {"impl_38"}
// Dependencies: {}
impl < IO > AsyncBufRead for TlsStream < IO > where IO : AsyncRead + AsyncWrite + Unpin , { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { match self . state { # [cfg (feature = "early-data")] TlsState :: EarlyData (..) => { self . get_mut () . poll_early_data (cx) ; Poll :: Pending } TlsState :: Stream | TlsState :: WriteShutdown => { let this = self . get_mut () ; let stream = Stream :: new (& mut this . io , & mut this . session) . set_eof (! this . state . readable ()) ; match stream . poll_fill_buf (cx) { Poll :: Ready (Ok (buf)) => { if buf . is_empty () { this . state . shutdown_read () ; } Poll :: Ready (Ok (buf)) } Poll :: Ready (Err (err)) if err . kind () == io :: ErrorKind :: ConnectionAborted => { this . state . shutdown_read () ; Poll :: Ready (Err (err)) } output => output , } } TlsState :: ReadShutdown | TlsState :: FullyShutdown => Poll :: Ready (Ok (& [])) , } } fn consume (mut self : Pin < & mut Self > , amt : usize) { self . session . reader () . consume (amt) ; } }
};
}
