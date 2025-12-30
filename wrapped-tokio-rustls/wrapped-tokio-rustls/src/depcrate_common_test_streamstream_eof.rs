// Generated macro for stream_eof (function)
macro_rules! Depcrate_common_test_streamstream_eof {
() => {
// Module: crate::common::test_stream
// Provides: {"stream_eof"}
// Dependencies: {}
# [tokio :: test] async fn stream_eof () -> io :: Result < () > { let (server , mut client) = make_pair () ; let mut server = Connection :: from (server) ; poll_fn (| cx | do_handshake (& mut client , & mut server , cx)) . await ? ; let mut bad = Expected (Cursor :: new (Vec :: new ())) ; let mut stream = Stream :: new (& mut bad , & mut client) ; let mut buf = Vec :: new () ; let result = stream . read_to_end (& mut buf) . await ; assert_eq ! (result . err () . map (| e | e . kind ()) , Some (io :: ErrorKind :: UnexpectedEof)) ; Ok (()) as io :: Result < () > }
};
}
