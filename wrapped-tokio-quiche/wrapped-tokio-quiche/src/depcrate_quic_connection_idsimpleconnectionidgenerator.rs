// Generated macro for SimpleConnectionIdGenerator (struct)
macro_rules! Depcrate_quic_connection_idSimpleConnectionIdGenerator {
() => {
// Module: crate::quic::connection::id
// Provides: {"SimpleConnectionIdGenerator"}
// Dependencies: {}
# [doc = " A [`ConnectionIdGenerator`] which creates random 20-byte connection IDs."] # [doc = ""] # [doc = " Random bytes are pulled directly from the operating system to create an ID."] # [doc = " Any `socket_cookie` value is ignored."] # [derive (Debug , Clone , Default)] pub struct SimpleConnectionIdGenerator ;
};
}
