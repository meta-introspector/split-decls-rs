// Generated macro for IoState (struct)
macro_rules! Depcrate_common_stateIoState {
() => {
// Module: crate::common_state
// Provides: {"IoState"}
// Dependencies: {}
# [doc = " Values of this structure are returned from [`Connection::process_new_packets`]"] # [doc = " and tell the caller the current I/O state of the TLS connection."] # [doc = ""] # [doc = " [`Connection::process_new_packets`]: crate::Connection::process_new_packets"] # [derive (Debug , Eq , PartialEq)] pub struct IoState { tls_bytes_to_write : usize , plaintext_bytes_to_read : usize , peer_has_closed : bool , }
};
}
