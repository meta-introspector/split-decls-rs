// Generated macro for QuicConnectionStream (type)
macro_rules! DepcrateQuicConnectionStream {
() => {
// Module: crate
// Provides: {"QuicConnectionStream"}
// Dependencies: {}
# [doc = " A stream of accepted [`InitialQuicConnection`]s from a [`listen`] call."] # [doc = ""] # [doc = " Errors from processing the client's QUIC initials can also be emitted on"] # [doc = " this stream. These do not indicate that the listener itself has failed."] pub type QuicConnectionStream < M > = ReceiverStream < io :: Result < InitialQuicConnection < UdpSocket , M > > > ;
};
}
