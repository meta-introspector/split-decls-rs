// Generated macro for stream_buffered_handshake (function)
macro_rules! Depcrate_common_test_streamstream_buffered_handshake {
() => {
// Module: crate::common::test_stream
// Provides: {"stream_buffered_handshake"}
// Dependencies: {}
# [tokio :: test] async fn stream_buffered_handshake () -> io :: Result < () > { use tokio :: io :: BufWriter ; let (server , mut client) = make_pair () ; let mut server = Connection :: from (server) ; { let mut good = BufWriter :: new (Good (& mut server)) ; let mut stream = Stream :: new (& mut good , & mut client) ; let (r , w) = poll_fn (| cx | stream . handshake (cx)) . await ? ; assert ! (r > 0) ; assert ! (w > 0) ; poll_fn (| cx | stream . handshake (cx)) . await ? ; } assert ! (! server . is_handshaking ()) ; assert ! (! client . is_handshaking ()) ; Ok (()) as io :: Result < () > }
};
}
