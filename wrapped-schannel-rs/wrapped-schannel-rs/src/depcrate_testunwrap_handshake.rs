// Generated macro for unwrap_handshake (function)
macro_rules! Depcrate_testunwrap_handshake {
() => {
// Module: crate::test
// Provides: {"unwrap_handshake"}
// Dependencies: {}
fn unwrap_handshake < S > (e : HandshakeError < S >) -> io :: Error { match e { HandshakeError :: Failure (e) => e , HandshakeError :: Interrupted (_) => panic ! ("not an I/O error") , } }
};
}
