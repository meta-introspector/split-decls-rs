// Generated macro for impl_616 (impl)
macro_rules! Depcrate_common_stateimpl_616 {
() => {
// Module: crate::common_state
// Provides: {"impl_616"}
// Dependencies: {}
impl IoState { # [doc = " How many bytes could be written by [`Connection::write_tls`] if called"] # [doc = " right now.  A non-zero value implies [`CommonState::wants_write`]."] # [doc = ""] # [doc = " [`Connection::write_tls`]: crate::Connection::write_tls"] pub fn tls_bytes_to_write (& self) -> usize { self . tls_bytes_to_write } # [doc = " How many plaintext bytes could be obtained via [`std::io::Read`]"] # [doc = " without further I/O."] pub fn plaintext_bytes_to_read (& self) -> usize { self . plaintext_bytes_to_read } # [doc = " True if the peer has sent us a close_notify alert.  This is"] # [doc = " the TLS mechanism to securely half-close a TLS connection,"] # [doc = " and signifies that the peer will not send any further data"] # [doc = " on this connection."] # [doc = ""] # [doc = " This is also signalled via returning `Ok(0)` from"] # [doc = " [`std::io::Read`], after all the received bytes have been"] # [doc = " retrieved."] pub fn peer_has_closed (& self) -> bool { self . peer_has_closed } }
};
}
