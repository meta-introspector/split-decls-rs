// Generated macro for ConnectionShutdownBehaviour (struct)
macro_rules! Depcrate_quic_connectionConnectionShutdownBehaviour {
() => {
// Module: crate::quic::connection
// Provides: {"ConnectionShutdownBehaviour"}
// Dependencies: {}
# [doc = " Parameters to close a [quiche::Connection]."] # [doc = ""] # [doc = " The connection will use these parameters for the `CONNECTION_CLOSE` frame"] # [doc = " it sends to its peer."] # [derive (Debug , Clone)] pub struct ConnectionShutdownBehaviour { # [doc = " Whether to send an application close or a regular close to the peer."] # [doc = ""] # [doc = " If this is true but the connection is not in a state where it is safe to"] # [doc = " send an application error (not established nor in early data), in"] # [doc = " accordance with [RFC 9000](https://www.rfc-editor.org/rfc/rfc9000.html#section-10.2.3-3), the"] # [doc = " error code is changed to `APPLICATION_ERROR` and the reason phrase is"] # [doc = " cleared."] pub send_application_close : bool , # [doc = " The [QUIC][proto-err] or [application-level][app-err] error code to send"] # [doc = " to the peer."] # [doc = ""] # [doc = " [proto-err]: https://www.rfc-editor.org/rfc/rfc9000.html#section-20.1"] # [doc = " [app-err]: https://www.rfc-editor.org/rfc/rfc9000.html#section-20.2"] pub error_code : u64 , # [doc = " The reason phrase to send to the peer."] pub reason : Vec < u8 > , }
};
}
