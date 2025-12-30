// Generated macro for stream_handshake (function)
macro_rules! Depcrate_common_test_streamstream_handshake {
() => {
// Module: crate::common::test_stream
// Provides: {"stream_handshake"}
// Dependencies: {}
# [tokio :: test] async fn stream_handshake () -> io :: Result < () > { let (server , mut client) = make_pair () ; let mut server = Connection :: from (server) ; { let mut good = Good (& mut server) ; let mut stream = Stream :: new (& mut good , & mut client) ; let (r , w) = poll_fn (| cx | stream . handshake (cx)) . await ? ; assert ! (r > 0) ; assert ! (w > 0) ; poll_fn (| cx | stream . handshake (cx)) . await ? ; } assert ! (! server . is_handshaking ()) ; assert ! (! client . is_handshaking ()) ; Ok (()) as io :: Result < () > }
};
}
