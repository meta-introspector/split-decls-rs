// Generated macro for do_handshake_step (function)
macro_rules! Depcratedo_handshake_step {
() => {
// Module: crate
// Provides: {"do_handshake_step"}
// Dependencies: {}
fn do_handshake_step (buffers : & mut TempBuffers , client : & mut ClientConnection , server : & mut ServerConnection ,) -> bool { if server . is_handshaking () || client . is_handshaking () { transfer (buffers , client , server , None) ; transfer (buffers , server , client , None) ; true } else { false } }
};
}
