// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl < S > AsyncWrite for TlsStream < S > where S : AsyncRead + AsyncWrite + Unpin , { fn poll_write (mut self : Pin < & mut Self > , ctx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { self . with_context (ctx , | s | s . write (buf)) } fn poll_flush (mut self : Pin < & mut Self > , ctx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . with_context (ctx , | s | s . flush ()) } fn poll_shutdown (mut self : Pin < & mut Self > , ctx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . with_context (ctx , | s | s . shutdown ()) } }
};
}
