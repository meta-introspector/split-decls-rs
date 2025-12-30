// Generated macro for terminal_size (function)
macro_rules! Depcrate_unixterminal_size {
() => {
// Module: crate::unix
// Provides: {"terminal_size"}
// Dependencies: {}
# [doc = " Returns the size of the terminal."] # [doc = ""] # [doc = " This function checks the stdout, stderr, and stdin streams (in that order)."] # [doc = " The size of the first stream that is a TTY will be returned.  If nothing"] # [doc = " is a TTY, then `None` is returned."] pub fn terminal_size () -> Option < (Width , Height) > { if let Some (size) = terminal_size_of (std :: io :: stdout ()) { Some (size) } else if let Some (size) = terminal_size_of (std :: io :: stderr ()) { Some (size) } else if let Some (size) = terminal_size_of (std :: io :: stdin ()) { Some (size) } else { None } }
};
}
