// Generated macro for impl_132 (impl)
macro_rules! Depcrate_serverimpl_132 {
() => {
// Module: crate::server
// Provides: {"impl_132"}
// Dependencies: {}
impl < IO > StartHandshake < IO > where IO : AsyncRead + AsyncWrite + Unpin , { # [doc = " Create a new object from an `IO` transport and prior TLS metadata."] pub fn from_parts (accepted : rustls :: server :: Accepted , transport : IO) -> Self { Self { accepted , io : transport , } } pub fn client_hello (& self) -> rustls :: server :: ClientHello < '_ > { self . accepted . client_hello () } pub fn into_stream (self , config : Arc < ServerConfig >) -> Accept < IO > { self . into_stream_with (config , | _ | ()) } pub fn into_stream_with < F > (self , config : Arc < ServerConfig > , f : F) -> Accept < IO > where F : FnOnce (& mut ServerConnection) , { let mut conn = match self . accepted . into_connection (config) { Ok (conn) => conn , Err ((error , alert)) => { return Accept (MidHandshake :: SendAlert { io : self . io , alert , error : io :: Error :: new (io :: ErrorKind :: InvalidData , error) , }) ; } } ; f (& mut conn) ; Accept (MidHandshake :: Handshaking (TlsStream { session : conn , io : self . io , state : TlsState :: Stream , need_flush : false , })) } }
};
}
