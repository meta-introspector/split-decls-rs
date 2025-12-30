// Generated macro for HandshakeHash (struct)
macro_rules! Depcrate_hash_hsHandshakeHash {
() => {
// Module: crate::hash_hs
// Provides: {"HandshakeHash"}
// Dependencies: {}
# [doc = " This deals with keeping a running hash of the handshake"] # [doc = " payloads.  This is computed by buffering initially.  Once"] # [doc = " we know what hash function we need to use we switch to"] # [doc = " incremental hashing."] # [doc = ""] # [doc = " For client auth, we also need to buffer all the messages."] # [doc = " This is disabled in cases where client auth is not possible."] pub (crate) struct HandshakeHash { provider : & 'static dyn hash :: Hash , ctx : Box < dyn hash :: Context > , # [doc = " buffer for client-auth."] client_auth : Option < Vec < u8 > > , }
};
}
