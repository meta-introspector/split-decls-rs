// Generated macro for HandshakeHashBuffer (struct)
macro_rules! Depcrate_hash_hsHandshakeHashBuffer {
() => {
// Module: crate::hash_hs
// Provides: {"HandshakeHashBuffer"}
// Dependencies: {}
# [doc = " Early stage buffering of handshake payloads."] # [doc = ""] # [doc = " Before we know the hash algorithm to use to verify the handshake, we just buffer the messages."] # [doc = " During the handshake, we may restart the transcript due to a HelloRetryRequest, reverting"] # [doc = " from the `HandshakeHash` to a `HandshakeHashBuffer` again."] # [derive (Clone)] pub (crate) struct HandshakeHashBuffer { buffer : Vec < u8 > , client_auth_enabled : bool , }
};
}
