// Generated macro for stream_handshake_write_eof (function)
macro_rules! Depcrate_common_test_streamstream_handshake_write_eof {
() => {
// Module: crate::common::test_stream
// Provides: {"stream_handshake_write_eof"}
// Dependencies: {}
# [tokio :: test] async fn stream_handshake_write_eof () -> io :: Result < () > { let (_ , mut client) = make_pair () ; let mut io = Eof ; let mut stream = Stream :: new (& mut io , & mut client) ; let mut cx = Context :: from_waker (noop_waker_ref ()) ; let r = stream . handshake (& mut cx) ; assert_eq ! (r . map_err (| err | err . kind ()) , Poll :: Ready (Err (io :: ErrorKind :: WriteZero))) ; Ok (()) as io :: Result < () > }
};
}
