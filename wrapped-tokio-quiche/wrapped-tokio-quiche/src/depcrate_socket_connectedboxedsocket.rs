// Generated macro for BoxedSocket (type)
macro_rules! Depcrate_socket_connectedBoxedSocket {
() => {
// Module: crate::socket::connected
// Provides: {"BoxedSocket"}
// Dependencies: {}
# [doc = " A type-erased variant of [`Socket`] with boxed `Tx` and `Rx` halves."] pub type BoxedSocket = Socket < Box < dyn DatagramSocketSend + Send + 'static > , Box < dyn DatagramSocketRecv + Sync + 'static > , > ;
};
}
