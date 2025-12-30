// Generated macro for do_handshake (function)
macro_rules! Depcratedo_handshake {
() => {
// Module: crate
// Provides: {"do_handshake"}
// Dependencies: {}
pub fn do_handshake (client : & mut impl DerefMut < Target = ConnectionCommon < impl SideData > > , server : & mut impl DerefMut < Target = ConnectionCommon < impl SideData > > ,) -> (usize , usize) { let (mut to_client , mut to_server) = (0 , 0) ; while server . is_handshaking () || client . is_handshaking () { to_server += transfer (client , server) ; server . process_new_packets () . unwrap () ; to_client += transfer (server , client) ; client . process_new_packets () . unwrap () ; } (to_server , to_client) }
};
}
