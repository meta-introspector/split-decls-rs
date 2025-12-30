// Generated macro for impl_127 (impl)
macro_rules! Depcrate_serverimpl_127 {
() => {
// Module: crate::server
// Provides: {"impl_127"}
// Dependencies: {}
impl TlsAcceptor { # [inline] pub fn accept < IO > (& self , stream : IO) -> Accept < IO > where IO : AsyncRead + AsyncWrite + Unpin , { self . accept_with (stream , | _ | ()) } pub fn accept_with < IO , F > (& self , stream : IO , f : F) -> Accept < IO > where IO : AsyncRead + AsyncWrite + Unpin , F : FnOnce (& mut ServerConnection) , { let mut session = match ServerConnection :: new (self . inner . clone ()) { Ok (session) => session , Err (error) => { return Accept (MidHandshake :: Error { io : stream , error : io :: Error :: new (io :: ErrorKind :: Other , error) , }) ; } } ; f (& mut session) ; Accept (MidHandshake :: Handshaking (TlsStream { session , io : stream , state : TlsState :: Stream , need_flush : false , })) } # [doc = " Get a read-only reference to underlying config"] pub fn config (& self) -> & Arc < ServerConfig > { & self . inner } }
};
}
