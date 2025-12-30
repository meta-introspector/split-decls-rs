// Generated macro for macro_2 (macro)
macro_rules! Depcrate_b_archmacro_2 {
() => {
// Module: crate::b_arch
// Provides: {"macro_2"}
// Dependencies: {}
# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "x86_64"))] windows_link :: link ! ("user32.dll" "system" fn GetWindowLongPtrW (hwnd : HWND , nindex : WINDOW_LONG_PTR_INDEX) -> isize) ;
};
}
