// Generated macro for do_handshake_until_both_error (function)
macro_rules! Depcratedo_handshake_until_both_error {
() => {
// Module: crate
// Provides: {"do_handshake_until_both_error"}
// Dependencies: {}
pub fn do_handshake_until_both_error (client : & mut ClientConnection , server : & mut ServerConnection ,) -> Result < () , Vec < ErrorFromPeer > > { match do_handshake_until_error (client , server) { Err (server_err @ ErrorFromPeer :: Server (_)) => { let mut errors = vec ! [server_err] ; transfer (server , client) ; let client_err = client . process_new_packets () . map_err (ErrorFromPeer :: Client) . expect_err ("client didn't produce error after server error") ; errors . push (client_err) ; Err (errors) } Err (client_err @ ErrorFromPeer :: Client (_)) => { let mut errors = vec ! [client_err] ; transfer (client , server) ; let server_err = server . process_new_packets () . map_err (ErrorFromPeer :: Server) . expect_err ("server didn't produce error after client error") ; errors . push (server_err) ; Err (errors) } Ok (()) => Ok (()) , } }
};
}
