// Generated macro for QuicCommand (enum)
macro_rules! Depcrate_quic_connectionQuicCommand {
() => {
// Module: crate::quic::connection
// Provides: {"QuicCommand"}
// Dependencies: {}
# [doc = " A command to execute on a [quiche::Connection] in the context of an"] # [doc = " [`ApplicationOverQuic`]."] # [doc = ""] # [doc = " We expect most [`ApplicationOverQuic`] implementations (such as"] # [doc = " [H3Driver](crate::http3::driver::H3Driver)) will provide some way to submit"] # [doc = " actions for them to take, for example via a channel. This enum may be"] # [doc = " accepted as part of those actions to inspect or alter the state of the"] # [doc = " underlying connection."] pub enum QuicCommand { # [doc = " Close the connection with the given parameters."] # [doc = ""] # [doc = " Some packets may still be sent after this command has been executed, so"] # [doc = " the worker task may continue running for a bit. See"] # [doc = " [`quiche::Connection::close`] for details."] ConnectionClose (ConnectionShutdownBehaviour) , # [doc = " Execute a custom callback on the connection."] Custom (Box < dyn FnOnce (& mut QuicheConnection) + Send + 'static >) , # [doc = " Collect the current [`SocketStats`] from the connection."] # [doc = ""] # [doc = " Unlike [`QuicConnection::stats()`], these statistics are not cached and"] # [doc = " instead are retrieved right before the command is executed."] Stats (Box < dyn FnOnce (datagram_socket :: SocketStats) + Send + 'static >) , }
};
}
