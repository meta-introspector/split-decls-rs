// Generated macro for can_trim_this (function)
macro_rules! Depcrate_compiler_rustcan_trim_this {
() => {
// Module: crate::compiler::rust
// Provides: {"can_trim_this"}
// Dependencies: {}
# [cfg (feature = "dist-client")] fn can_trim_this (input_path : & Path) -> bool { trace ! ("can_trim_this: input_path={:?}" , input_path) ; let mut ar_path = input_path . to_path_buf () ; ar_path . set_extension ("a") ; input_path . extension () . map (| e | e == RLIB_EXTENSION) . unwrap_or (false) && ! ar_path . exists () }
};
}
