// Generated macro for do_handshake_altered (function)
macro_rules! Depcratedo_handshake_altered {
() => {
// Module: crate
// Provides: {"do_handshake_altered"}
// Dependencies: {}
pub fn do_handshake_altered (client : ClientConnection , alter_server_message : impl Fn (& mut Message < '_ >) -> Altered , alter_client_message : impl Fn (& mut Message < '_ >) -> Altered , server : ServerConnection ,) -> Result < () , ErrorFromPeer > { let mut client : Connection = Connection :: Client (client) ; let mut server : Connection = Connection :: Server (server) ; while server . is_handshaking () || client . is_handshaking () { transfer_altered (& mut client , & alter_client_message , & mut server) ; server . process_new_packets () . map_err (ErrorFromPeer :: Server) ? ; transfer_altered (& mut server , & alter_server_message , & mut client) ; client . process_new_packets () . map_err (ErrorFromPeer :: Client) ? ; } Ok (()) }
};
}
