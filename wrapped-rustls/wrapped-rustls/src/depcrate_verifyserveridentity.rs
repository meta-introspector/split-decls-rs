// Generated macro for ServerIdentity (struct)
macro_rules! Depcrate_verifyServerIdentity {
() => {
// Module: crate::verify
// Provides: {"ServerIdentity"}
// Dependencies: {}
# [doc = " Data required to verify a server's identity."] # [non_exhaustive] # [derive (Debug)] pub struct ServerIdentity < 'a > { # [doc = " Identity information presented by the server."] pub identity : & 'a Identity < 'a > , # [doc = " The server name the client specified when connecting to the server."] pub server_name : & 'a ServerName < 'a > , # [doc = " OCSP response stapled to the server's `Certificate` message, if any."] # [doc = ""] # [doc = " Empty if no OCSP response was received, and that also"] # [doc = " covers the case where `request_ocsp_response()` returns false."] pub ocsp_response : & 'a [u8] , # [doc = " Current time against which time-sensitive inputs should be validated."] pub now : UnixTime , }
};
}
