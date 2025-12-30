// Generated macro for stream_write_zero (function)
macro_rules! Depcrate_common_test_streamstream_write_zero {
() => {
// Module: crate::common::test_stream
// Provides: {"stream_write_zero"}
// Dependencies: {}
# [tokio :: test] async fn stream_write_zero () -> io :: Result < () > { let (server , mut client) = make_pair () ; let mut server = Connection :: from (server) ; poll_fn (| cx | do_handshake (& mut client , & mut server , cx)) . await ? ; let mut io = Eof ; let mut stream = Stream :: new (& mut io , & mut client) ; stream . write_all (b"1") . await . unwrap () ; let result = stream . flush () . await ; assert_eq ! (result . err () . map (| e | e . kind ()) , Some (io :: ErrorKind :: WriteZero)) ; Ok (()) as io :: Result < () > }
};
}
