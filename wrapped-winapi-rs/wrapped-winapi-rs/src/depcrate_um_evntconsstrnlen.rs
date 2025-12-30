// Generated macro for strnlen (function)
macro_rules! Depcrate_um_evntconsstrnlen {
() => {
// Module: crate::um::evntcons
// Provides: {"strnlen"}
// Dependencies: {}
# [inline] unsafe fn strnlen (s : PCSTR , max_len : isize) -> isize { let mut len = 0 ; while * s . offset (len) != 0 && len < max_len { len += 1 } len }
};
}
