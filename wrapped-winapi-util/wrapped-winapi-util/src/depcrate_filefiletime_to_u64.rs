// Generated macro for filetime_to_u64 (function)
macro_rules! Depcrate_filefiletime_to_u64 {
() => {
// Module: crate::file
// Provides: {"filetime_to_u64"}
// Dependencies: {}
fn filetime_to_u64 (t : FILETIME) -> Option < u64 > { let v = ((t . dwHighDateTime as u64) << 32) | (t . dwLowDateTime as u64) ; if v == 0 { None } else { Some (v) } }
};
}
