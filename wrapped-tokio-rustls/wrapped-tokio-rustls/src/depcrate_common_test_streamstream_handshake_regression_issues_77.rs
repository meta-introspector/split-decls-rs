// Generated macro for stream_handshake_regression_issues_77 (function)
macro_rules! Depcrate_common_test_streamstream_handshake_regression_issues_77 {
() => {
// Module: crate::common::test_stream
// Provides: {"stream_handshake_regression_issues_77"}
// Dependencies: {}
# [tokio :: test] async fn stream_handshake_regression_issues_77 () -> io :: Result < () > { let (_ , mut client) = make_pair () ; let mut bad = Expected (Cursor :: new (b"\x15\x03\x01\x00\x02\x02\x00" . to_vec ())) ; let mut stream = Stream :: new (& mut bad , & mut client) ; let mut cx = Context :: from_waker (noop_waker_ref ()) ; let r = stream . handshake (& mut cx) ; assert_eq ! (r . map_err (| err | err . kind ()) , Poll :: Ready (Err (io :: ErrorKind :: InvalidData))) ; Ok (()) as io :: Result < () > }
};
}
