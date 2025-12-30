// Generated macro for stream_bad (function)
macro_rules! Depcrate_common_test_streamstream_bad {
() => {
// Module: crate::common::test_stream
// Provides: {"stream_bad"}
// Dependencies: {}
# [tokio :: test] async fn stream_bad () -> io :: Result < () > { let (server , mut client) = make_pair () ; let mut server = Connection :: from (server) ; poll_fn (| cx | do_handshake (& mut client , & mut server , cx)) . await ? ; client . set_buffer_limit (Some (1024)) ; let mut bad = Pending ; let mut stream = Stream :: new (& mut bad , & mut client) ; assert_eq ! (poll_fn (| cx | stream . as_mut_pin () . poll_write (cx , & [0x42 ; 8])) . await ?, 8) ; assert_eq ! (poll_fn (| cx | stream . as_mut_pin () . poll_write (cx , & [0x42 ; 8])) . await ?, 8) ; let r = poll_fn (| cx | stream . as_mut_pin () . poll_write (cx , & [0x00 ; 1024])) . await ? ; assert ! (r < 1024) ; let mut cx = Context :: from_waker (noop_waker_ref ()) ; let ret = stream . as_mut_pin () . poll_write (& mut cx , & [0x01]) ; assert ! (ret . is_pending ()) ; Ok (()) as io :: Result < () > }
};
}
