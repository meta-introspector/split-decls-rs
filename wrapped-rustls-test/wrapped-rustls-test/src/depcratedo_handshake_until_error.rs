// Generated macro for do_handshake_until_error (function)
macro_rules! Depcratedo_handshake_until_error {
() => {
// Module: crate
// Provides: {"do_handshake_until_error"}
// Dependencies: {}
pub fn do_handshake_until_error (client : & mut ClientConnection , server : & mut ServerConnection ,) -> Result < () , ErrorFromPeer > { while server . is_handshaking () || client . is_handshaking () { transfer (client , server) ; server . process_new_packets () . map_err (ErrorFromPeer :: Server) ? ; transfer (server , client) ; client . process_new_packets () . map_err (ErrorFromPeer :: Client) ? ; } Ok (()) }
};
}
