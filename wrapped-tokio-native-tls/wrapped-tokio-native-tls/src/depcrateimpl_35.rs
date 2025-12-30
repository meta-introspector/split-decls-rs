// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl TlsConnector { # [doc = " Connects the provided stream with this connector, assuming the provided"] # [doc = " domain."] # [doc = ""] # [doc = " This function will internally call `TlsConnector::connect` to connect"] # [doc = " the stream and returns a future representing the resolution of the"] # [doc = " connection operation. The returned future will resolve to either"] # [doc = " `TlsStream<S>` or `Error` depending if it's successful or not."] # [doc = ""] # [doc = " This is typically used for clients who have already established, for"] # [doc = " example, a TCP connection to a remote server. That stream is then"] # [doc = " provided here to perform the client half of a connection to a"] # [doc = " TLS-powered server."] pub async fn connect < S > (& self , domain : & str , stream : S) -> Result < TlsStream < S > , Error > where S : AsyncRead + AsyncWrite + Unpin , { handshake (move | s | self . 0 . connect (domain , s) , stream) . await } }
};
}
