// Generated macro for is_a_tty (function)
macro_rules! Depcrateis_a_tty {
() => {
// Module: crate
// Provides: {"is_a_tty"}
// Dependencies: {}
fn is_a_tty (stream : Stream) -> bool { use std :: io :: IsTerminal ; match stream { Stream :: Stdout => std :: io :: stdout () . is_terminal () , Stream :: Stderr => std :: io :: stderr () . is_terminal () , } }
};
}
