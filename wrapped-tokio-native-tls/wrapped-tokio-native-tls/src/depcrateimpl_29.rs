// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < S > AsyncRead for TlsStream < S > where S : AsyncRead + AsyncWrite + Unpin , { fn poll_read (mut self : Pin < & mut Self > , ctx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { self . with_context (ctx , | s | { let n = s . read (buf . initialize_unfilled ()) ? ; buf . advance (n) ; Ok (()) }) } }
};
}
