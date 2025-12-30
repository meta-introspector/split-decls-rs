// Generated macro for terminal_size_of (function)
macro_rules! Depcrate_unixterminal_size_of {
() => {
// Module: crate::unix
// Provides: {"terminal_size_of"}
// Dependencies: {}
# [doc = " Returns the size of the terminal using the given file descriptor, if available."] # [doc = ""] # [doc = " If the given file descriptor is not a tty, returns `None`"] pub fn terminal_size_of < Fd : AsFd > (fd : Fd) -> Option < (Width , Height) > { use rustix :: termios :: { isatty , tcgetwinsize } ; if ! isatty (& fd) { return None ; } let winsize = tcgetwinsize (& fd) . ok () ? ; let rows = winsize . ws_row ; let cols = winsize . ws_col ; if rows > 0 && cols > 0 { Some ((Width (cols) , Height (rows))) } else { None } }
};
}
