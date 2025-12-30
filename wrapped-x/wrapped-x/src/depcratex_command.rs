// Generated macro for x_command (function)
macro_rules! Depcratex_command {
() => {
// Module: crate
// Provides: {"x_command"}
// Dependencies: {}
# [cfg (not (any (windows , unix)))] fn x_command (_dir : & Path) -> Command { compile_error ! ("Unsupported platform") ; }
};
}
