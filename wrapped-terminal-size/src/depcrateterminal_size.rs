// Generated macro for terminal_size (function)
macro_rules! Depcrateterminal_size {
() => {
// Module: crate
// Provides: {"terminal_size"}
// Dependencies: {}
# [cfg (not (any (unix , windows)))] pub fn terminal_size () -> Option < (Width , Height) > { None }
};
}
