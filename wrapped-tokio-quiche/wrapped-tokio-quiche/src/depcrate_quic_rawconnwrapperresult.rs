// Generated macro for ConnWrapperResult (struct)
macro_rules! Depcrate_quic_rawConnWrapperResult {
() => {
// Module: crate::quic::raw
// Provides: {"ConnWrapperResult"}
// Dependencies: {}
# [doc = " Result of manually wrapping a [`quiche::Connection`] in an"] # [doc = " [`InitialQuicConnection`]."] # [doc = ""] # [doc = " This struct bundles the interfaces which interact with the connection."] pub struct ConnWrapperResult < Tx , M > where Tx : DatagramSocketSend + Send + 'static + ? Sized , M : Metrics , { # [doc = " The connection wrapper."] pub conn : InitialQuicConnection < Tx , M > , # [doc = " Sender for inbound packets on the connection."] pub incoming_tx : mpsc :: Sender < Incoming > , # [doc = " Receiver for `connection closed` notifications. This fires"] # [doc = " after a `CONNECTION_CLOSE` frame has been sent on the connection,"] # [doc = " but before `worker_shutdown_rx`."] pub conn_close_rx : ConnCloseReceiver , # [doc = " Receiver which fires only when its associated sender is dropped."] # [doc = " This happens when the connection's IO task exits."] pub worker_shutdown_rx : mpsc :: Receiver < () > , }
};
}
