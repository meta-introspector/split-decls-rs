// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl TlsAcceptor { # [doc = " Accepts a new client connection with the provided stream."] # [doc = ""] # [doc = " This function will internally call `TlsAcceptor::accept` to connect"] # [doc = " the stream and returns a future representing the resolution of the"] # [doc = " connection operation. The returned future will resolve to either"] # [doc = " `TlsStream<S>` or `Error` depending if it's successful or not."] # [doc = ""] # [doc = " This is typically used after a new socket has been accepted from a"] # [doc = " `TcpListener`. That socket is then passed to this function to perform"] # [doc = " the server half of accepting a client connection."] pub async fn accept < S > (& self , stream : S) -> Result < TlsStream < S > , Error > where S : AsyncRead + AsyncWrite + Unpin , { handshake (move | s | self . 0 . accept (s) , stream) . await } }
};
}
