// Generated macro for do_handshake (function)
macro_rules! Depcrate_common_test_streamdo_handshake {
() => {
// Module: crate::common::test_stream
// Provides: {"do_handshake"}
// Dependencies: {}
fn do_handshake (client : & mut ClientConnection , server : & mut Connection , cx : & mut Context < '_ > ,) -> Poll < io :: Result < () > > { let mut good = Good (server) ; let mut stream = Stream :: new (& mut good , client) ; while stream . session . is_handshaking () { ready ! (stream . handshake (cx)) ? ; } while stream . session . wants_write () { ready ! (stream . write_io (cx)) ? ; } Poll :: Ready (Ok (())) }
};
}
