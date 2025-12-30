// Generated macro for stream_good_impl (function)
macro_rules! Depcrate_common_test_streamstream_good_impl {
() => {
// Module: crate::common::test_stream
// Provides: {"stream_good_impl"}
// Dependencies: {}
async fn stream_good_impl (vectored : bool , bufread : bool) -> io :: Result < () > { const FILE : & [u8] = include_bytes ! ("../../README.md") ; let (server , mut client) = make_pair () ; let mut server = Connection :: from (server) ; poll_fn (| cx | do_handshake (& mut client , & mut server , cx)) . await ? ; io :: copy (& mut Cursor :: new (FILE) , & mut server . writer ()) ? ; server . send_close_notify () ; { let mut good = Good (& mut server) ; let mut stream = Stream :: new (& mut good , & mut client) ; let mut buf = Vec :: new () ; if bufread { dbg ! (tokio :: io :: copy_buf (& mut stream , & mut buf) . await) ? ; } else { dbg ! (stream . read_to_end (& mut buf) . await) ? ; } assert_eq ! (buf , FILE) ; dbg ! (utils :: write (& mut stream , b"Hello World!" , vectored) . await) ? ; stream . session . send_close_notify () ; dbg ! (stream . shutdown () . await) ? ; } let mut buf = String :: new () ; dbg ! (server . process_new_packets ()) . map_err (| e | io :: Error :: new (io :: ErrorKind :: Other , e)) ? ; dbg ! (server . reader () . read_to_string (& mut buf)) ? ; assert_eq ! (buf , "Hello World!") ; Ok (()) as io :: Result < () > }
};
}
