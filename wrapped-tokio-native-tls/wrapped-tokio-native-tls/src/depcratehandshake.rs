// Generated macro for handshake (function)
macro_rules! Depcratehandshake {
() => {
// Module: crate
// Provides: {"handshake"}
// Dependencies: {}
async fn handshake < F , S > (f : F , stream : S) -> Result < TlsStream < S > , Error > where F : FnOnce (AllowStd < S > ,) -> Result < native_tls :: TlsStream < AllowStd < S > > , HandshakeError < AllowStd < S > > > + Unpin , S : AsyncRead + AsyncWrite + Unpin , { let start = StartedHandshakeFuture (Some (StartedHandshakeFutureInner { f , stream })) ; match start . await { Err (e) => Err (e) , Ok (StartedHandshake :: Done (s)) => Ok (s) , Ok (StartedHandshake :: Mid (s)) => MidHandshake (Some (s)) . await , } }
};
}
