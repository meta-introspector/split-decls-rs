// Generated macro for Terminal (struct)
macro_rules! Depcrate_terminalTerminal {
() => {
// Module: crate::terminal
// Provides: {"Terminal"}
// Dependencies: {}
# [derive (Debug)] pub struct Terminal < B > where B : Backend , { pub backend : B , buffers : [Buffer ; 2] , current : usize , hidden_cursor : bool , known_size : Rect , }
};
}
