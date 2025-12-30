// Generated macro for HandshakeKind (enum)
macro_rules! Depcrate_common_stateHandshakeKind {
() => {
// Module: crate::common_state
// Provides: {"HandshakeKind"}
// Dependencies: {}
# [doc = " Describes which sort of handshake happened."] # [derive (Debug , PartialEq , Clone , Copy)] # [non_exhaustive] pub enum HandshakeKind { # [doc = " A full handshake."] # [doc = ""] # [doc = " This is the typical TLS connection initiation process when resumption is"] # [doc = " not yet unavailable, and the initial `ClientHello` was accepted by the server."] Full , # [doc = " A full TLS1.3 handshake, with an extra round-trip for a `HelloRetryRequest`."] # [doc = ""] # [doc = " The server can respond with a `HelloRetryRequest` if the initial `ClientHello`"] # [doc = " is unacceptable for several reasons, the most likely being if no supported key"] # [doc = " shares were offered by the client."] FullWithHelloRetryRequest , # [doc = " A resumed handshake."] # [doc = ""] # [doc = " Resumed handshakes involve fewer round trips and less cryptography than"] # [doc = " full ones, but can only happen when the peers have previously done a full"] # [doc = " handshake together, and then remember data about it."] Resumed , # [doc = " A resumed handshake, with an extra round-trip for a `HelloRetryRequest`."] # [doc = ""] # [doc = " The server can respond with a `HelloRetryRequest` if the initial `ClientHello`"] # [doc = " is unacceptable for several reasons, but this does not prevent the client"] # [doc = " from resuming."] ResumedWithHelloRetryRequest , }
};
}
