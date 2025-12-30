// Generated macro for stream_handshake_eof (function)
macro_rules! Depcrate_common_test_streamstream_handshake_eof {
() => {
// Module: crate::common::test_stream
// Provides: {"stream_handshake_eof"}
// Dependencies: {}
# [tokio :: test] async fn stream_handshake_eof () -> io :: Result < () > { let (_ , mut client) = make_pair () ; let mut bad = Expected (Cursor :: new (Vec :: new ())) ; let mut stream = Stream :: new (& mut bad , & mut client) ; let mut cx = Context :: from_waker (noop_waker_ref ()) ; let r = stream . handshake (& mut cx) ; assert_eq ! (r . map_err (| err | err . kind ()) , Poll :: Ready (Err (io :: ErrorKind :: UnexpectedEof))) ; Ok (()) as io :: Result < () > }
};
}
